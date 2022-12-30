use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error generating HTTP request: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Error parsing response: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Unexpected error.")]
    Unknown,
}
