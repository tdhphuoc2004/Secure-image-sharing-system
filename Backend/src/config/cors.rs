use actix_cors::Cors;
use std::env;

pub fn create_cors() -> Cors {
    let allowed_origins = env::var("CORS_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000,http://127.0.0.1:3000".to_string());
    
    Cors::default()
        .allowed_origin_fn(move |origin, _req_head| {
            allowed_origins.split(',').any(|allowed| {
                origin.as_bytes() == allowed.as_bytes()
            })
        })
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec!["content-type", "authorization", "x-requested-with"])
        .supports_credentials()
        .max_age(3600)
}