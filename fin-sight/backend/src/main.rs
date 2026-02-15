use axum::{
    routing::get,
    Router,
};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create the application router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/health", get(health_check));

    // Get port from environment variable or default to 3000
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server starting on port {}", port);

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "FinSight Backend is running!"
}
