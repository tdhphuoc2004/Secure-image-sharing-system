use actix_web::web;
use crate::handlers::health::health_check;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/health", web::get().to(health_check))
            // Future routes will be added here:
            // .configure(auth_routes::configure)
            // .configure(image_routes::configure)
    );
}