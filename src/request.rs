use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};

use crate::app_error::AppError;

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub(crate) async fn parse_http_request(raw: &[u8]) -> Result<Request, AppError> {
    let mut lines: Vec<String> = Vec::new();
    let mut reader = BufReader::new(raw);

    loop {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .await
            .map_err(|_| AppError::MalformedRequest("Invalid http request.".to_string()))?;

        let trimmed = line.trim_end_matches(|c| c == '\r' || c == '\n');
        if trimmed.is_empty() {
            break;
        }
        lines.push(trimmed.to_string());
    }

    let request_line = lines.get(0).ok_or(AppError::MalformedRequest(
        "Invalid http request.".to_string(),
    ))?;
    let mut parts = request_line.splitn(3, ' ');
    let method = parts
        .next()
        .ok_or(AppError::MalformedRequest(
            "Invalid http request.".to_string(),
        ))?
        .to_string();
    let path = parts
        .next()
        .ok_or(AppError::MalformedRequest(
            "Invalid http request.".to_string(),
        ))?
        .to_string();
    let version = parts
        .next()
        .ok_or(AppError::MalformedRequest(
            "Invalid http request.".to_string(),
        ))?
        .to_string();

    let mut headers: HashMap<String, String> = HashMap::new();

    for line in &lines[1..] {
        if let Some((k, v)) = line.split_once(':') {
            headers.insert(
                k.trim().to_string().to_lowercase(),
                v.trim().to_string().to_lowercase(),
            );
        }
    }

    let body: Vec<u8> = if let Some(len) = headers.get("content-length") {
        let len = len.parse().unwrap_or(0);
        if len > 10_000_000 {
            return Err(AppError::PayloadTooLarge {
                max_bytes: 10_000_000,
            });
        }
        let mut body = vec![0u8; len];
        reader
            .read_exact(&mut body)
            .await
            .map_err(|_| AppError::IncompleteBody)?;
        body
    } else {
        vec![]
    };

    let params: HashMap<String, String> = HashMap::new();
    let bodys = String::from_utf8_lossy(raw);
    println!("{}", bodys);

    Ok(Request {
        method,
        path,
        headers,
        version,
        params,
        body,
    })
}
