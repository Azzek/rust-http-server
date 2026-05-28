use std::collections::HashMap;
use tokio::io::AsyncWriteExt;

#[derive(PartialEq)]
pub struct StatusCode(pub u16);

impl StatusCode {
    // --- 1xx Informational ---
    pub const CONTINUE: Self = Self(100);
    pub const SWITCHING_PROTOCOLS: Self = Self(101);
    pub const PROCESSING: Self = Self(102);
    pub const EARLY_HINTS: Self = Self(103);

    // --- 2xx Success ---
    pub const OK: Self = Self(200);
    pub const CREATED: Self = Self(201);
    pub const ACCEPTED: Self = Self(202);
    pub const NON_AUTHORITATIVE_INFORMATION: Self = Self(203);
    pub const NO_CONTENT: Self = Self(204);
    pub const RESET_CONTENT: Self = Self(205);
    pub const PARTIAL_CONTENT: Self = Self(206);
    pub const MULTI_STATUS: Self = Self(207);
    pub const ALREADY_REPORTED: Self = Self(208);
    pub const IM_USED: Self = Self(226);

    // --- 3xx Redirection ---
    pub const MULTIPLE_CHOICES: Self = Self(300);
    pub const MOVED_PERMANENTLY: Self = Self(301);
    pub const FOUND: Self = Self(302);
    pub const SEE_OTHER: Self = Self(303);
    pub const NOT_MODIFIED: Self = Self(304);
    pub const USE_PROXY: Self = Self(305);
    pub const TEMPORARY_REDIRECT: Self = Self(307);
    pub const PERMANENT_REDIRECT: Self = Self(308);

    // --- 4xx Client Error ---
    pub const BAD_REQUEST: Self = Self(400);
    pub const UNAUTHORIZED: Self = Self(401);
    pub const PAYMENT_REQUIRED: Self = Self(402);
    pub const FORBIDDEN: Self = Self(403);
    pub const NOT_FOUND: Self = Self(404);
    pub const METHOD_NOT_ALLOWED: Self = Self(405);
    pub const NOT_ACCEPTABLE: Self = Self(406);
    pub const PROXY_AUTHENTICATION_REQUIRED: Self = Self(407);
    pub const REQUEST_TIMEOUT: Self = Self(408);
    pub const CONFLICT: Self = Self(409);
    pub const GONE: Self = Self(410);
    pub const LENGTH_REQUIRED: Self = Self(411);
    pub const PRECONDITION_FAILED: Self = Self(412);
    pub const PAYLOAD_TOO_LARGE: Self = Self(413);
    pub const URI_TOO_LONG: Self = Self(414);
    pub const UNSUPPORTED_MEDIA_TYPE: Self = Self(415);
    pub const RANGE_NOT_SATISFIABLE: Self = Self(416);
    pub const EXPECTATION_FAILED: Self = Self(417);
    pub const IM_A_TEAPOT: Self = Self(418); // RFC 2324 (Easter Egg)
    pub const MISDIRECTED_REQUEST: Self = Self(421);
    pub const UNPROCESSABLE_ENTITY: Self = Self(422);
    pub const LOCKED: Self = Self(423);
    pub const FAILED_DEPENDENCY: Self = Self(424);
    pub const TOO_EARLY: Self = Self(425);
    pub const UPGRADE_REQUIRED: Self = Self(426);
    pub const PRECONDITION_REQUIRED: Self = Self(428);
    pub const TOO_MANY_REQUESTS: Self = Self(429);
    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: Self = Self(431);
    pub const UNAVAILABLE_FOR_LEGAL_REASONS: Self = Self(451);

    // --- 5xx Server Error ---
    pub const INTERNAL_SERVER_ERROR: Self = Self(500);
    pub const NOT_IMPLEMENTED: Self = Self(501);
    pub const BAD_GATEWAY: Self = Self(502);
    pub const SERVICE_UNAVAILABLE: Self = Self(503);
    pub const GATEWAY_TIMEOUT: Self = Self(504);
    pub const HTTP_VERSION_NOT_SUPPORTED: Self = Self(505);
    pub const VARIANT_ALSO_NEGOTIATES: Self = Self(506);
    pub const INSUFFICIENT_STORAGE: Self = Self(507);
    pub const LOOP_DETECTED: Self = Self(508);
    pub const NOT_EXTENDED: Self = Self(510);
    pub const NETWORK_AUTHENTICATION_REQUIRED: Self = Self(511);

    pub fn code(&self) -> u16 {
        self.0
    }

    pub fn reason_phrase(&self) -> &'static str {
        match *self {
            // 1xx
            Self::CONTINUE => "Continue",
            Self::SWITCHING_PROTOCOLS => "Switching Protocols",
            Self::PROCESSING => "Processing",
            Self::EARLY_HINTS => "Early Hints",

            // 2xx
            Self::OK => "OK",
            Self::CREATED => "Created",
            Self::ACCEPTED => "Accepted",
            Self::NON_AUTHORITATIVE_INFORMATION => "Non-Authoritative Information",
            Self::NO_CONTENT => "No Content",
            Self::RESET_CONTENT => "Reset Content",
            Self::PARTIAL_CONTENT => "Partial Content",
            Self::MULTI_STATUS => "Multi-Status",
            Self::ALREADY_REPORTED => "Already Reported",
            Self::IM_USED => "IM Used",

            // 3xx
            Self::MULTIPLE_CHOICES => "Multiple Choices",
            Self::MOVED_PERMANENTLY => "Moved Permanently",
            Self::FOUND => "Found",
            Self::SEE_OTHER => "See Other",
            Self::NOT_MODIFIED => "Not Modified",
            Self::USE_PROXY => "Use Proxy",
            Self::TEMPORARY_REDIRECT => "Temporary Redirect",
            Self::PERMANENT_REDIRECT => "Permanent Redirect",

            // 4xx
            Self::BAD_REQUEST => "Bad Request",
            Self::UNAUTHORIZED => "Unauthorized",
            Self::PAYMENT_REQUIRED => "Payment Required",
            Self::FORBIDDEN => "Forbidden",
            Self::NOT_FOUND => "Not Found",
            Self::METHOD_NOT_ALLOWED => "Method Not Allowed",
            Self::NOT_ACCEPTABLE => "Not Acceptable",
            Self::PROXY_AUTHENTICATION_REQUIRED => "Proxy Authentication Required",
            Self::REQUEST_TIMEOUT => "Request Timeout",
            Self::CONFLICT => "Conflict",
            Self::GONE => "Gone",
            Self::LENGTH_REQUIRED => "Length Required",
            Self::PRECONDITION_FAILED => "Precondition Failed",
            Self::PAYLOAD_TOO_LARGE => "Payload Too Large",
            Self::URI_TOO_LONG => "URI Too Long",
            Self::UNSUPPORTED_MEDIA_TYPE => "Unsupported Media Type",
            Self::RANGE_NOT_SATISFIABLE => "Range Not Satisfiable",
            Self::EXPECTATION_FAILED => "Expectation Failed",
            Self::IM_A_TEAPOT => "I'm a teapot",
            Self::MISDIRECTED_REQUEST => "Misdirected Request",
            Self::UNPROCESSABLE_ENTITY => "Unprocessable Entity",
            Self::LOCKED => "Locked",
            Self::FAILED_DEPENDENCY => "Failed Dependency",
            Self::TOO_EARLY => "Too Early",
            Self::UPGRADE_REQUIRED => "Upgrade Required",
            Self::PRECONDITION_REQUIRED => "Precondition Required",
            Self::TOO_MANY_REQUESTS => "Too Many Requests",
            Self::REQUEST_HEADER_FIELDS_TOO_LARGE => "Request Header Fields Too Large",
            Self::UNAVAILABLE_FOR_LEGAL_REASONS => "Unavailable For Legal Reasons",

            // 5xx
            Self::INTERNAL_SERVER_ERROR => "Internal Server Error",
            Self::NOT_IMPLEMENTED => "Not Implemented",
            Self::BAD_GATEWAY => "Bad Gateway",
            Self::SERVICE_UNAVAILABLE => "Service Unavailable",
            Self::GATEWAY_TIMEOUT => "Gateway Timeout",
            Self::HTTP_VERSION_NOT_SUPPORTED => "HTTP Version Not Supported",
            Self::VARIANT_ALSO_NEGOTIATES => "Variant Also Negotiates",
            Self::INSUFFICIENT_STORAGE => "Insufficient Storage",
            Self::LOOP_DETECTED => "Loop Detected",
            Self::NOT_EXTENDED => "Not Extended",
            Self::NETWORK_AUTHENTICATION_REQUIRED => "Network Authentication Required",

            _ => "Unknown Status Code",
        }
    }
}

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for (StatusCode, String) {
    fn into_response(self) -> Response {
        let mut res = Response::new(self.0);
        res.body = self.1.into_bytes();
        res
    }
}

pub struct Response {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: vec![],
        }
    }

    pub async fn write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: AsyncWriteExt + Unpin,
    {
        let code = self.status.code();
        let reason = self.status.reason_phrase();

        if code >= 400 {
            println!("[SERVER LOG] Error response sent: {}", reason);
        }
        writer
            .write_all(format!("HTTP/1.1 {} {}\r\n", code, reason).as_bytes())
            .await?;

        for (k, v) in &self.headers {
            writer
                .write_all(format!("{}: {}\r\n", k, v).as_bytes())
                .await?;
        }

        writer.write_all(b"\r\n").await?;
        writer.write_all(&self.body).await?;
        writer.flush().await
    }
}
