use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use crate::models::shared_links::SharedLink;

// Response DTOs - synchronized with SharedLink model
#[derive(Debug, Serialize)]
pub struct SharedLinkResponse {
    pub id: i32,
    pub image_id: i32,
    pub share_token: String,
    pub created_at: SystemTime,
    pub expires_at: Option<SystemTime>,
    pub max_views: Option<i32>,
    pub current_views: i32,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct CreateSharedLinkResponse {
    pub share_token: String,
    pub share_url: String, // Full URL for easy sharing
    pub expires_at: Option<SystemTime>,
    pub max_views: Option<i32>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SharedLinkListResponse {
    pub links: Vec<SharedLinkResponse>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

// Conversion implementation - all fields match
impl From<SharedLink> for SharedLinkResponse {
    fn from(link: SharedLink) -> Self {
        SharedLinkResponse {
            id: link.id,
            image_id: link.image_id,
            share_token: link.share_token,
            created_at: link.created_at,
            expires_at: link.expires_at,
            max_views: link.max_views,
            current_views: link.current_views,
            is_active: link.is_active,
        }
    }
}

impl CreateSharedLinkResponse {
    pub fn new(
        share_token: String,
        base_url: &str,
        expires_at: Option<SystemTime>,
        max_views: Option<i32>,
    ) -> Self {
        Self {
            share_url: format!("{}/shared/{}", base_url, share_token),
            share_token,
            expires_at,
            max_views,
            message: "Shared link created successfully".to_string(),
        }
    }
}