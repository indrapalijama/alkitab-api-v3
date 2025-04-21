use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to fetch data from external source: {0}")]
    ExternalApiError(String),

    #[error("Invalid book name: {0}")]
    InvalidBookError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Resource not found: {0}")]
    NotFound(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::ExternalApiError(msg) => {
                HttpResponse::BadGateway().json(json!({
                    "error": "External API Error",
                    "message": msg
                }))
            }
            AppError::InvalidBookError(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "Invalid Book",
                    "message": msg
                }))
            }
            AppError::InvalidInput(msg) => {
                HttpResponse::BadRequest().json(json!({
                    "error": "Invalid Input",
                    "message": msg
                }))
            }
            AppError::ExternalService(msg) => {
                HttpResponse::BadGateway().json(json!({
                    "error": "External Service Error",
                    "message": msg
                }))
            }
            AppError::NotFound(msg) => {
                HttpResponse::NotFound().json(json!({
                    "error": "Not Found",
                    "message": msg
                }))
            }
        }
    }
} 