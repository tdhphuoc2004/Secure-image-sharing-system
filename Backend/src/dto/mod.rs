// Request DTOs
pub mod user_request_dto;
pub mod image_request_dto;
pub mod shared_link_request_dto;

// Response DTOs
pub mod user_response_dto;
pub mod image_response_dto;
pub mod shared_link_response_dto;

// Common response structures
pub mod common_response_dto;

// Re-export commonly used DTOs for easier imports
pub use user_request_dto::*;
pub use user_response_dto::*;
pub use image_request_dto::*;
pub use image_response_dto::*;
pub use shared_link_request_dto::*;
pub use shared_link_response_dto::*;
pub use common_response_dto::*;