use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    database::create_connection,
    models::{CreateUser, User},
    repositories::{DbConnection, UserRepository},
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/users", post(create_user))
        .route("/api/users/:id", get(get_user))
}

async fn health_check() -> &'static str {
    "FinSight Backend is running!"
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub message: String,
    pub user: User,
}

async fn create_user(
    State(db_conn): State<Arc<DbConnection>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, StatusCode> {
    let create_user = CreateUser {
        email: payload.email,
        password: payload.password,
    };

    let user = db_conn.create(&create_user).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(CreateUserResponse {
        message: "User created successfully".to_string(),
        user,
    }))
}

async fn get_user(
    State(db_conn): State<Arc<DbConnection>>,
    Path(id): Path<String>,
) -> Result<Json<User>, StatusCode> {
    let user = db_conn.find_by_id(id.parse().map_err(|_| StatusCode::BAD_REQUEST)?)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(StatusCode::NOT_FOUND),
    }
}