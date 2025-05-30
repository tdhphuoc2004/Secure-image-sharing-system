use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub message: String,
    pub error_code: Option<String>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct PaginationResponse<T> {
    pub data: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}

impl ErrorResponse {
    pub fn new(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            error_code: None,
            details: None,
        }
    }

    pub fn with_code(message: &str, error_code: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            error_code: Some(error_code.to_string()),
            details: None,
        }
    }
}