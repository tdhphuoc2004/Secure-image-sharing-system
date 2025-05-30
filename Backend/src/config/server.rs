use actix_web::{web, App, HttpServer, middleware::Logger};
use crate::database::connection::DbPool;
use crate::config::cors::create_cors;
use std::env;

pub async fn start_server(pool: DbPool) -> std::io::Result<()> {
    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let server_address = format!("{}:{}", server_host, server_port);
    
    println!("🌐 Starting HTTP server at http://{}", server_address);
    
    HttpServer::new(move || {
        App::new()
            .wrap(create_cors())
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(&server_address)?
    .run()
    .await
}