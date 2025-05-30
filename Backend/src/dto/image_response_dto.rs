use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use crate::models::Image;

// Response DTOs
#[derive(Debug, Serialize)]
pub struct ImageResponse {
    pub id: i32,
    pub filename: String,
    pub original_filename: String,
    pub file_size: i64,
    pub mime_type: String,
    pub upload_time: SystemTime,
    pub is_public: bool,
}

#[derive(Debug, Serialize)]
pub struct ImageUploadResponse {
    pub id: i32,
    pub filename: String,
    pub upload_url: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ImageListResponse {
    pub images: Vec<ImageResponse>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

impl From<Image> for ImageResponse {
    fn from(image: Image) -> Self {
        ImageResponse {
            id: image.id,
            filename: image.filename,
            original_filename: image.original_filename,
            file_size: image.file_size,
            mime_type: image.mime_type,
            upload_time: image.upload_time,
            is_public: image.is_public,
        }
    }
}