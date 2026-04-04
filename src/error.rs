use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenArcError {
    #[error("HTTP Request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("API returned an error: {status_code} - {message}")]
    ApiError {
        status_code: u16,
        message: String,
    },
}
