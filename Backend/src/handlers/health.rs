use actix_web::{web, HttpResponse, Result};
use crate::database::connection::{test_connection, DbPool};
use serde_json::json;
use std::env;

pub async fn health_check(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    match test_connection(&pool).await {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "status": "healthy",
            "database": "connected",
            "timestamp": chrono::Utc::now(),
            "service": "SecureShareSystem",
            "version": env!("CARGO_PKG_VERSION")
        }))),
        Err(e) => {
            log::error!("Health check failed: {}", e);
            Ok(HttpResponse::ServiceUnavailable().json(json!({
                "status": "unhealthy",
                "database": "disconnected",
                "error": "Database connection failed",
                "timestamp": chrono::Utc::now(),
                "service": "SecureShareSystem"
            })))
        }
    }
}