use thiserror::Error;

#[derive(Error, Debug)]
pub enum YahooError {
    #[error("Failed to processing with external api; `{0}`")]
    ExternalServiceError(String),
    #[error("Network error occurred; `{0}`")]
    NetworkError(String),
    #[error("Invalid response; `{0}`")]
    InvalidResponse(String),
    #[error("Invalid call; `{0}`")]
    InvalidCall(String),
    #[error("Failed to parse response; `{0}`")]
    ParseError(String),
    #[error("Unknown error; `{0}`")]
    Unknown(String),
}
