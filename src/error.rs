use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Reqwest(reqwest::Error),
    ApiError(ApiError),
    RateLimitExceeded,
}

#[derive(Debug, Clone)]
pub struct ApiError {
    pub status_code: StatusCode,
    pub error_code: i32,
    pub message: String,
}

#[derive(Deserialize)]
pub(crate) struct ErrorResponse {
    pub code: i32,
    pub message: String,
    #[serde(default)]
    pub retry_after: Option<u64>,
}