/// Shared types and response structures for the API
use serde::{Deserialize, Serialize};

/// Standard API response structure
#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub message: String,
    pub alert: String,
}

impl ApiResponse {
    pub fn success(message: String) -> Self {
        Self {
            message,
            alert: "success".to_string(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            message,
            alert: "error".to_string(),
        }
    }
}

/// API response with additional data payload
#[derive(Serialize, Deserialize)]
pub struct ApiResponseWithData<T> {
    pub message: String,
    pub alert: String,
    pub data: Option<T>,
}

impl<T> ApiResponseWithData<T> {
    pub fn success(message: String, data: T) -> Self {
        Self {
            message,
            alert: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            message,
            alert: "error".to_string(),
            data: None,
        }
    }
}
