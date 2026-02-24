use std::fmt;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    Network(String),
    Http { status: u16, message: String },
    Config(String),
    Parse(String),
    HttpClient(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Network(msg) => write!(f, "Network error: {}", msg),
            ApiError::Http { status, message } => {
                write!(f, "HTTP error {}: {}", status, message)
            }
            ApiError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ApiError::HttpClient(msg) => write!(f, "HTTP client error: {}", msg),
            ApiError::Config(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        ApiError::Network(error.to_string())
    }
}
