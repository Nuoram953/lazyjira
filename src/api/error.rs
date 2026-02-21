use std::fmt;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    Network(String),
    Http { status: u16, message: String },
    Parse(String),
    Authentication(String),
    RateLimited { retry_after: Option<u64> },
    HttpClient(String),
    InvalidConfiguration(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Network(msg) => write!(f, "Network error: {}", msg),
            ApiError::Http { status, message } => {
                write!(f, "HTTP error {}: {}", status, message)
            }
            ApiError::Parse(msg) => write!(f, "Parse error: {}", msg),
            ApiError::Authentication(msg) => write!(f, "Authentication error: {}", msg),
            ApiError::RateLimited { retry_after } => {
                if let Some(seconds) = retry_after {
                    write!(f, "Rate limited. Retry after {} seconds", seconds)
                } else {
                    write!(f, "Rate limited")
                }
            }
            ApiError::HttpClient(msg) => write!(f, "HTTP client error: {}", msg),
            ApiError::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

// Convert from anyhow::Error if needed
impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        ApiError::Network(error.to_string())
    }
}
