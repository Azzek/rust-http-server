use crate::commands::server::response::{IntoResponse, Response, StatusCode};
use std::io::{self};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("malformed http request: {0}")]
    MalformedRequest(String),

    #[error("incomplete request body")]
    IncompleteBody,

    #[error("payload too large: maximum allowed is {max_bytes} bytes")]
    PayloadTooLarge { max_bytes: usize },

    #[error("network i/o error: {0}")]
    Io(#[from] io::Error),

    #[error("http version or method not supported")]
    NotImplemented,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::MalformedRequest(msg) => {
                let mut res = Response::new(StatusCode::BAD_REQUEST);
                res.headers
                    .insert("content-type".to_string(), "application/json".to_string());
                res.body = format!("{{\"error\": \"Bad Request\", \"message\": \"{}\"}}", msg)
                    .into_bytes();
                res
            }
            AppError::IncompleteBody => {
                let mut res = Response::new(StatusCode::BAD_REQUEST);
                res.body = b"{\"error\": \"Incomplete Body\"}".to_vec();
                res
            }
            AppError::PayloadTooLarge { max_bytes } => {
                let mut res = Response::new(StatusCode::PAYLOAD_TOO_LARGE);
                res.headers
                    .insert("content-type".to_string(), "application/json".to_string());
                res.body = format!(
                    "{{\"error\": \"Payload Too Large\", \"max_bytes\": {}}}",
                    max_bytes
                )
                .into_bytes();
                res
            }
            AppError::NotImplemented => Response::new(StatusCode::NOT_IMPLEMENTED),
            AppError::Io(_) => Response::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
