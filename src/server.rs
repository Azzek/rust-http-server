use crate::commands::server::{
    app,
    request::Request,
    response::{Response, StatusCode},
};

pub async fn start_listening(port: u16) {
    let mut app = app::App::new(port);

    app.get("/xd", hello_world).post("/users", post_hello_world);

    app.listen().await;
}

async fn hello_world(req: Request) -> Response {
    Response::new(StatusCode::OK)
}

async fn post_hello_world(req: Request) -> Response {
    Response::new(StatusCode::OK)
}
