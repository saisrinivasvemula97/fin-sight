use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use std::env;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use sqlx::SqlitePool;

mod database;
mod models;
mod repositories;

// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub user_repo: repositories::UserRepository,
    pub account_repo: repositories::AccountRepository,
    pub category_repo: repositories::CategoryRepository,
    pub transaction_repo: repositories::TransactionRepository,
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create SQLite connection pool
    let pool = match database::create_pool().await {
        Ok(pool) => {
            tracing::info!("SQLite connection pool created successfully");
            
            // Test database connection
            if let Err(e) = database::test_connection(&pool).await {
                tracing::warn!("SQLite connection test failed: {}", e);
            }
            
            pool
        }
        Err(e) => {
            tracing::error!("Failed to create SQLite connection pool: {}", e);
            std::process::exit(1);
        }
    };

    // Create repositories
    let user_repo = repositories::UserRepository::new(repositories::DbConnection::new(pool.clone()));
    let account_repo = repositories::AccountRepository::new(repositories::DbConnection::new(pool.clone()));
    let category_repo = repositories::CategoryRepository::new(repositories::DbConnection::new(pool.clone()));
    let transaction_repo = repositories::TransactionRepository::new(repositories::DbConnection::new(pool.clone()));

    // Create shared application state
    let app_state = AppState {
        user_repo,
        account_repo,
        category_repo,
        transaction_repo,
    };

    // Create the application router with CORS
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/health", get(health_check))
        // User routes
        .route("/api/users", post(create_user))
        .route("/api/users/{id}", get(get_user))
        // Account routes
        .route("/api/accounts", post(create_account))
        .route("/api/accounts/{id}", get(get_account))
        .route("/api/accounts/user/{user_id}", get(get_accounts_by_user))
        .route("/api/accounts/{id}/balance", post(update_balance))
        // Category routes
        .route("/api/categories", get(get_categories))
        .route("/api/categories/{id}", get(get_category))
        .route("/api/categories", post(create_category))
        // Transaction routes
        .route("/api/transactions", post(create_transaction))
        .route("/api/transactions/{id}", get(get_transaction))
        .route("/api/transactions/account/{account_id}", get(get_transactions_by_account))
        .route("/api/transactions/user/{user_id}", get(get_transactions_by_user))
        .route("/api/transactions/{id}", post(update_transaction))
        .route("/api/transactions/{id}", axum::routing::delete(delete_transaction))
        // Analytics routes
        .route("/api/analytics/category-summary", get(get_category_summary))
        .route("/api/analytics/net-worth/{user_id}", get(get_net_worth))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        )
        .with_state(app_state);

    // Get port from environment variable or default to 3000
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to address");

    tracing::info!("FinSight Backend starting on port {}", port);

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "FinSight Backend is running!"
}

// Import handlers from separate modules
use axum::{
    extract::{Path, State, Json, Query},
    http::StatusCode,
    response::Json as AxumJson,
};

use crate::models::*;

// User handlers
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<AxumJson<ApiResponse<User>>, StatusCode> {
    let user = state.user_repo.create(&payload).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(AxumJson(ApiResponse::success(user)))
}

async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<AxumJson<ApiResponse<User>>, StatusCode> {
    let user = state.user_repo.find_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match user {
        Some(user) => Ok(AxumJson(ApiResponse::success(user))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Account handlers
async fn create_account(
    State(state): State<AppState>,
    Json(payload): Json<CreateAccount>,
) -> Result<AxumJson<ApiResponse<Account>>, StatusCode> {
    let account = state.account_repo.create(&payload).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(AxumJson(ApiResponse::success(account)))
}

async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<AxumJson<ApiResponse<Account>>, StatusCode> {
    let account = state.account_repo.find_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match account {
        Some(account) => Ok(AxumJson(ApiResponse::success(account))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_accounts_by_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<AxumJson<ApiResponse<Vec<Account>>>, StatusCode> {
    let accounts = state.account_repo.find_by_user_id(&user_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(AxumJson(ApiResponse::success(accounts)))
}

async fn update_balance(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAccount>,
) -> Result<AxumJson<ApiResponse<Account>>, StatusCode> {
    let account = state.account_repo.update_balance(
        &id,
        payload.current_balance.unwrap_or(0.0),
        payload.available_balance,
    ).await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(AxumJson(ApiResponse::success(account)))
}

// Category handlers
async fn get_categories(
    State(state): State<AppState>,
) -> Result<AxumJson<ApiResponse<Vec<Category>>>, StatusCode> {
    let categories = state.category_repo.find_all().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(AxumJson(ApiResponse::success(categories)))
}

async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<AxumJson<ApiResponse<Category>>, StatusCode> {
    let category = state.category_repo.find_by_id(id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match category {
        Some(category) => Ok(AxumJson(ApiResponse::success(category))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategory>,
) -> Result<AxumJson<ApiResponse<Category>>, StatusCode> {
    let category = state.category_repo.create(&payload).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(AxumJson(ApiResponse::success(category)))
}

// Transaction handlers
async fn create_transaction(
    State(state): State<AppState>,
    Json(payload): Json<CreateTransaction>,
) -> Result<AxumJson<ApiResponse<Transaction>>, StatusCode> {
    let transaction = state.transaction_repo.create(&payload).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(AxumJson(ApiResponse::success(transaction)))
}

async fn get_transaction(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<AxumJson<ApiResponse<Transaction>>, StatusCode> {
    let transaction = state.transaction_repo.find_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match transaction {
        Some(transaction) => Ok(AxumJson(ApiResponse::success(transaction))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_transactions_by_account(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
) -> Result<AxumJson<ApiResponse<Vec<Transaction>>>, StatusCode> {
    let transactions = state.transaction_repo.find_by_account_id(&account_id, 100, 0).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(AxumJson(ApiResponse::success(transactions)))
}

async fn get_transactions_by_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<AxumJson<ApiResponse<Vec<Transaction>>>, StatusCode> {
    let transactions = state.transaction_repo.find_by_user_id(&user_id, 100, 0).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(AxumJson(ApiResponse::success(transactions)))
}

async fn update_transaction(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTransaction>,
) -> Result<AxumJson<ApiResponse<Transaction>>, StatusCode> {
    let transaction = state.transaction_repo.update(&id, &payload).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(AxumJson(ApiResponse::success(transaction)))
}

async fn delete_transaction(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<AxumJson<ApiResponse<()>>, StatusCode> {
    state.transaction_repo.delete(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(AxumJson(ApiResponse::success(())))
}

// Analytics handlers
async fn get_category_summary(
    State(state): State<AppState>,
    Query(params): Query<CategoryQuery>,
) -> Result<AxumJson<ApiResponse<Vec<CategorySummary>>>, StatusCode> {
    let summary = state.transaction_repo.get_category_summary(
        &params.category_type.unwrap_or_default(),
        None,
        None,
    ).await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(AxumJson(ApiResponse::success(summary)))
}

async fn get_net_worth(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<AxumJson<ApiResponse<NetWorth>>, StatusCode> {
    let net_worth = state.transaction_repo.get_net_worth(&user_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(AxumJson(ApiResponse::success(net_worth)))
}
