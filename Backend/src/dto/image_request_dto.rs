use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewImage {
    pub user_id: i32,
    pub filename: String, //Actual file name on server
    pub original_filename: String, //Original file name uploaded by user
    pub file_size: i64,
    pub mime_type: String,
    pub image_location: String,
    pub AES_encryption_key_encrypted: String,
    pub file_hash: String,
    pub iv: String,
    pub expiration_time: Option<SystemTime>,
    pub is_public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageUploadRequest {
    pub filename: String,
    pub expiration_time: Option<SystemTime>,
    pub is_public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateImageRequest {
    pub is_public: Option<bool>,
    pub expiration_time: Option<SystemTime>,
}

