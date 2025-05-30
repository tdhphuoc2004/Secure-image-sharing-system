mod database;
mod models;
mod dto;
mod handlers;
mod config;

use config::server::start_server;
use database::setup::initialize_database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment and logging
    dotenv::dotenv().ok();
    env_logger::init();
    
    println!("🚀 Starting SecureShareSystem...");
    
    // Initialize database
    let pool = initialize_database().await
        .expect("Failed to initialize database");
    
    // Start server
    start_server(pool).await
}