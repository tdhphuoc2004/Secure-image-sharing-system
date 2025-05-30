use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::env;

pub type DbPool = PgPool;

pub async fn create_pool() -> Result<DbPool, sqlx::Error> 
{
    // Get individual database configuration from environment variables
    let db_host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let db_username = env::var("DB_USERNAME").unwrap_or_else(|_| "postgres".to_string());
    let db_password = env::var("DB_PASSWORD").unwrap_or_else(|_| "password".to_string());
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "secure_share_db".to_string());
    let max_connections = env::var("DB_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "10".to_string())
        .parse::<u32>()
        .unwrap_or(10);
    
    // Construct the database URL
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_username, db_password, db_host, db_port, db_name
    );
    
    // Log connection info (hide password for security)
    let safe_url = format!(
        "postgresql://{}:***@{}:{}/{}",
        db_username, db_host, db_port, db_name
    );

    println!("🔗 Database URL: {}", database_url);
    println!("📊 Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await?;

    println!("✅ Database connection pool created");
    Ok(pool)
}

pub async fn test_connection(pool: &DbPool) -> Result<(), sqlx::Error> {
    let row = sqlx::query("SELECT 1 as test, version() as version")
        .fetch_one(pool)
        .await?;
    
    let test_value: i32 = row.get("test");
    let version: String = row.get("version");
    
    if test_value == 1 {
        println!("✅ Database test query successful");
        println!("📊 PostgreSQL version: {}", version);
        Ok(())
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}