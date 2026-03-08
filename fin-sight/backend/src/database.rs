use sqlx::{SqlitePool, Row, Error as SqlxError};
use std::env;

/// Create a SQLite connection pool
pub async fn create_pool() -> Result<SqlitePool, SqlxError> {
    // Use in-memory database for simplicity
    let db_path = "sqlite::memory:";
    
    let pool = SqlitePool::connect(&db_path).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    Ok(pool)
}

/// Test database connection
pub async fn test_connection(pool: &SqlitePool) -> Result<(), SqlxError> {
    let _ = sqlx::query("SELECT 1").fetch_one(pool).await?;
    Ok(())
}