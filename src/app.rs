use crate::commands::server::{
    app_error::AppError,
    request::{Request, parse_http_request},
    response::{IntoResponse, Response, StatusCode},
    router::Router,
};
use std::{future::Future, net::SocketAddr, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub struct App {
    router: Router,
    port: u16,
}

impl App {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            router: Router { routes: Vec::new() },
        }
    }

    pub fn get<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        self.router.add("GET", path, handler);
        self
    }

    pub fn post<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        self.router.add("POST", path, handler);
        self
    }

    pub fn put<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        self.router.add("PUT", path, handler);
        self
    }

    pub fn delete<F, Fut>(&mut self, path: &str, handler: F) -> &mut Self
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        self.router.add("DELETE", path, handler);
        self
    }

    pub async fn listen(self) {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr).await.unwrap();
        println!("Listening on http://{}", addr);

        let router = Arc::new(self.router);

        loop {
            let (socket, addr) = listener.accept().await.unwrap();
            let router = router.clone();
            tokio::spawn(async move {
                process(socket, addr, router).await;
            });
        }
    }
}

async fn process(mut socket: TcpStream, addr: SocketAddr, router: Arc<Router>) {
    let mut request_buffer = Vec::new();
    let mut read_buffer = vec![0u8; 4096];

    const MAX_REQUEST_SIZE: usize = 10_000_000;

    loop {
        match socket.read(&mut read_buffer).await {
            Ok(0) => {
                println!("Client {} disconnected prematurely", addr);
                break;
            }
            Ok(n) => {
                request_buffer.extend_from_slice(&read_buffer[..n]);

                if request_buffer.len() > MAX_REQUEST_SIZE {
                    println!("[{}] Request too large, forcing disconnect", addr);
                    break;
                }

                match parse_http_request(&request_buffer).await {
                    Ok(request) => {
                        let version = request.version.to_lowercase();

                        if version != "http/1.1" && version != "http/1.0" {
                            println!("[{}] Unsupported HTTP version: {}", addr, request.version);

                            let response = Response::new(StatusCode::HTTP_VERSION_NOT_SUPPORTED);
                            let _ = response.write(&mut socket).await;
                            let _ = socket.shutdown().await;
                            break;
                        }

                        let response = router.dispatch(request).await;

                        if let Err(e) = response.write(&mut socket).await {
                            println!("[{}] Write error: {}", addr, e);
                        }

                        let _ = socket.shutdown().await;
                        break;
                    }
                    Err(AppError::IncompleteBody) => {
                        continue;
                    }
                    Err(err) => {
                        if let Err(e) = err.into_response().write(&mut socket).await {
                            println!("[{}] Failed to send error response: {}", addr, e);
                        }
                        break;
                    }
                }
            }
            Err(e) => {
                println!("[{}] Read error: {}", addr, e);
                break;
            }
        }
    }
}
