//! Error handling module for the NASA API Proxy Worker.
//! 
//! This module provides a comprehensive error type system that unifies
//! different error sources and provides consistent error responses.

use serde::{Deserialize, Serialize};
use worker::{Error as WorkerError, Response};

/// Type alias for Results using our custom error type.
pub type Result<T> = std::result::Result<T, NasaApiError>;

/// Standardized error response structure returned to API clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// The error type/category.
    pub error: String,
    /// Human-readable error message.
    pub message: String,
    /// HTTP status code.
    pub status_code: u16,
}

/// Main error type for the NASA API proxy.
/// 
/// This enum represents all possible errors that can occur within the application,
/// providing a unified error handling approach.
#[derive(Debug, thiserror::Error)]
pub enum NasaApiError {
    /// Cloudflare Worker-specific errors.
    #[error("Worker error: {0}")]
    Worker(String),
    
    /// HTTP request errors.
    #[error("Request error: {0}")]
    Request(String),
    
    /// Errors returned by NASA APIs.
    #[error("NASA API error: {0}")]
    NasaApi(String),
    
    /// Input validation errors.
    #[error("Validation error: {0}")]
    Validation(String),
    
    /// Authentication/authorization errors.
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    /// Rate limiting errors.
    #[error("Rate limit exceeded")]
    RateLimit,
    
    /// Cache operation errors.
    #[error("Cache error: {0}")]
    Cache(String),
    
    /// JSON serialization/deserialization errors.
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Resource not found errors.
    #[error("Not found: {0}")]
    NotFound(String),
    
    /// Bad request errors.
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    /// Internal server errors.
    #[error("Internal error: {0}")]
    Internal(String),
}

impl NasaApiError {
    /// Returns the appropriate HTTP status code for this error.
    pub fn status_code(&self) -> u16 {
        match self {
            NasaApiError::Validation(_) | NasaApiError::BadRequest(_) => 400,
            NasaApiError::Authentication(_) => 401,
            NasaApiError::NotFound(_) => 404,
            NasaApiError::RateLimit => 429,
            _ => 500,
        }
    }
    
    /// Converts the error into an HTTP Response with appropriate status code and JSON body.
    pub fn to_response(&self) -> Response {
        let error_response = ErrorResponse {
            error: format!("{self:?}").split("(").next().unwrap_or("Unknown").to_string(),
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
#[allow(dead_code)]
pub fn to_worker_result<T>(result: Result<T>) -> worker::Result<T> {
    result.map_err(|e| worker::Error::RustError(e.to_string()))
}

impl From<NasaApiError> for worker::Error {
    fn from(err: NasaApiError) -> Self {
        worker::Error::RustError(err.to_string())
    }
}