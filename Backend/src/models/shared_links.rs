use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SharedLink {
    pub id: i32,
    pub image_id: i32,
    pub share_token: String,
    pub created_at: SystemTime,
    pub expires_at: Option<SystemTime>,
    pub max_views: Option<i32>,
    pub current_views: i32,
    pub is_active: bool,
}