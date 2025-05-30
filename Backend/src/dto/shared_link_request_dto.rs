use serde::{Deserialize, Serialize};
use std::time::SystemTime;

// Input DTOs for creating shared links
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSharedLinkRequest {
    pub image_id: i32,
    pub recipient_email: Option<String>,
    pub expires_in_hours: Option<u64>, // More user-friendly than absolute time
    pub max_views: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSharedLinkRequest {
    pub expires_at: Option<SystemTime>,
    pub max_views: Option<i32>,
    pub is_active: Option<bool>,
}

// For internal use when creating the actual database record
#[derive(Debug, Serialize, Deserialize)]
pub struct NewSharedLink {
    pub image_id: i32,
    pub share_token: String,
    pub recipient_email: Option<String>,
    pub expires_at: Option<SystemTime>,
    pub max_views: Option<i32>,
}