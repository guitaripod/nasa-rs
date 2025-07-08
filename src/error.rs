use serde::{Deserialize, Serialize};
use worker::{Error as WorkerError, Response};

pub type Result<T> = std::result::Result<T, NasaApiError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status_code: u16,
}

#[derive(Debug, thiserror::Error)]
pub enum NasaApiError {
    #[error("Worker error: {0}")]
    Worker(String),
    
    #[error("Request error: {0}")]
    Request(String),
    
    #[error("NASA API error: {0}")]
    NasaApi(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("Cache error: {0}")]
    Cache(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl NasaApiError {
    pub fn status_code(&self) -> u16 {
        match self {
            NasaApiError::Validation(_) | NasaApiError::BadRequest(_) => 400,
            NasaApiError::Authentication(_) => 401,
            NasaApiError::NotFound(_) => 404,
            NasaApiError::RateLimit => 429,
            _ => 500,
        }
    }
    
    pub fn to_response(&self) -> Response {
        let error_response = ErrorResponse {
            error: format!("{:?}", self).split("(").next().unwrap_or("Unknown").to_string(),
            message: self.to_string(),
            status_code: self.status_code(),
        };
        
        Response::from_json(&error_response)
            .unwrap_or_else(|_| Response::error("Internal Server Error", 500).unwrap())
            .with_status(self.status_code())
    }
}

impl From<WorkerError> for NasaApiError {
    fn from(err: WorkerError) -> Self {
        NasaApiError::Worker(err.to_string())
    }
}

impl From<serde_json::Error> for NasaApiError {
    fn from(err: serde_json::Error) -> Self {
        NasaApiError::Serialization(err.to_string())
    }
}

impl From<reqwest::Error> for NasaApiError {
    fn from(err: reqwest::Error) -> Self {
        NasaApiError::Request(err.to_string())
    }
}

// Helper to convert our Result to worker::Result
pub fn to_worker_result<T>(result: Result<T>) -> worker::Result<T> {
    result.map_err(|e| worker::Error::RustError(e.to_string()))
}

impl From<NasaApiError> for worker::Error {
    fn from(err: NasaApiError) -> Self {
        worker::Error::RustError(err.to_string())
    }
}