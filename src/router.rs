use crate::commands::server::{
    request::Request,
    response::{Response, StatusCode},
};
use std::{collections::HashMap, pin::Pin, sync::Arc};

type BoxFuture = Pin<Box<dyn Future<Output = Response> + Send>>;

type Handler = Arc<dyn Fn(Request) -> BoxFuture + Send + Sync>;

#[derive(Clone)]
enum Segment {
    Lit(String),
    Param(String),
}

#[derive(Clone)]
pub(crate) struct Route {
    method: String,
    segments: Vec<Segment>,
    handler: Handler,
}

pub(crate) struct Router {
    pub(crate) routes: Vec<Route>,
}

impl Router {
    pub(crate) fn add<F, Fut>(&mut self, method: &str, path: &str, f: F)
    where
        F: Fn(Request) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let segments: Vec<Segment> = path
            .split("/")
            .filter(|s| !s.is_empty())
            .map(|s| {
                if s.starts_with(":") {
                    Segment::Param(s[1..].to_string())
                } else {
                    Segment::Lit(s.to_string())
                }
            })
            .collect();

        let handler: Handler = Arc::new(move |req| Box::pin(f(req)));

        let route = Route {
            method: method.to_string(),
            segments: segments,
            handler: handler,
        };

        self.routes.push(route);
    }

    pub(crate) async fn dispatch(&self, mut request: Request) -> Response {
        let clean_path = request.path.split('?').next().unwrap_or("/");

        let path_segments: Vec<&str> = clean_path.split('/').filter(|s| !s.is_empty()).collect();

        for route in self.routes.iter() {
            if route.method != request.method {
                continue;
            }
            if route.segments.len() != path_segments.len() {
                continue;
            }

            let mut params: HashMap<String, String> = HashMap::new();
            let mut matched = true;

            for (p_seg, r_seg) in route.segments.iter().zip(&path_segments) {
                match p_seg {
                    Segment::Lit(s) => {
                        if s != r_seg {
                            matched = false;
                            break;
                        }
                    }
                    Segment::Param(k) => {
                        params.insert(k.to_string(), r_seg.to_string());
                    }
                }
            }

            if matched {
                request.params = params;
                return (route.handler)(request).await;
            }
        }

        Response::new(StatusCode::NOT_FOUND)
    }
}
