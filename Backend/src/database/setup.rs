use crate::database::connection::{create_pool, test_connection, DbPool};

pub async fn initialize_database() -> Result<DbPool, Box<dyn std::error::Error>> {
    println!("🔗 Initializing database...");
    
    // Create connection pool
    let pool = create_pool().await?;
    
    // Test connection
    test_connection(&pool).await?;
    println!("✅ Database connection successful");
    
    // Run migrations
  //  run_migrations(&pool).await?;
    println!("✅ Database initialization complete");
    
    Ok(pool)
}

async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::Error> {
    println!("🔄 Running database migrations...");
    
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    Ok(())
}