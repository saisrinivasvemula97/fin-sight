use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::database::create_connection;

pub fn create_simple_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/test", get(test_db))
}

async fn health_check() -> &'static str {
    "FinSight Backend is running!"
}

async fn test_db() -> Result<Json<serde_json::Value>, StatusCode> {
    let conn = create_connection().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Test a simple query
    let mut stmt = conn.lock().unwrap().prepare("SELECT 1 as test").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let result: i32 = stmt.query_row([], |row| row.get(0)).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(serde_json::json!({
        "status": "success",
        "test_value": result,
        "message": "Database connection working!"
    })))
}