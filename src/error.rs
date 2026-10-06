use std::collections::HashMap;
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
    pub model_errors: Option<HashMap<String, Vec<ModelError>>>,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelError {
    pub message: String,
    #[serde(default)]
    pub max_length: Option<i32>,
    #[serde(default)]
    pub actual_length: Option<i32>,
}

#[derive(Deserialize)]
pub(crate) struct ErrorResponse {
    pub code: i32,
    pub message: String,
    #[serde(default)]
    pub errors: Option<HashMap<String, Vec<ModelError>>>,
    #[serde(default)]
    pub retry_after: Option<u64>,
}