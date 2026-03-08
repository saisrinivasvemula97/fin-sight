use fin_sight_backend::{AppState, database, models::*};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::{get, post},
    Router,
};
use serde_json::json;
use tower::ServiceExt;
use sqlx::SqlitePool;

#[tokio::test]
async fn test_health_check() {
    let pool = database::create_pool().await.unwrap();
    let user_repo = fin_sight_backend::repositories::UserRepository::new(
        fin_sight_backend::repositories::DbConnection::new(pool.clone())
    );
    let account_repo = fin_sight_backend::repositories::AccountRepository::new(
        fin_sight_backend::repositories::DbConnection::new(pool.clone())
    );
    let category_repo = fin_sight_backend::repositories::CategoryRepository::new(
        fin_sight_backend::repositories::DbConnection::new(pool.clone())
    );
    let transaction_repo = fin_sight_backend::repositories::TransactionRepository::new(
        fin_sight_backend::repositories::DbConnection::new(pool.clone())
    );

    let app_state = AppState {
        user_repo,
        account_repo,
        category_repo,
        transaction_repo,
    };

    let app = Router::new()
        .route("/api/health", get(fin_sight_backend::health_check))
        .with_state(app_state);

    let response = app
        .oneshot(Request::builder().uri("/api/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_categories() {
    let pool = database::create_pool().await.unwrap();
    let user_repo = fin_sight_backend::repositories::UserRepository::new(
        fin_sight_backend::repositories::DbConnection::new(pool.clone())
    );
    let account_repo = fin_sight_backend::repositories::AccountRepository::new(
        fin_sight_backend::repositories::DbConnection::new(pool.clone())
    );
    let category_repo = fin_sight_backend::repositories::CategoryRepository::new(
        fin_sight_backend::repositories::DbConnection::new(pool.clone())
    );
    let transaction_repo = fin_sight_backend::repositories::TransactionRepository::new(
        fin_sight_backend::repositories::DbConnection::new(pool.clone())
    );

    let app_state = AppState {
        user_repo,
        account_repo,
        category_repo,
        transaction_repo,
    };

    let app = Router::new()
        .route("/api/categories", get(|State(state): fin_sight_backend::axum::extract::State<AppState>| async {
            let categories = state.category_repo.find_all().await.unwrap();
            fin_sight_backend::axum::response::Json(fin_sight_backend::models::ApiResponse::success(categories))
        }))
        .with_state(app_state);

    let response = app
        .oneshot(Request::builder().uri("/api/categories").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_user() {
    let pool = database::create_pool().await.unwrap();
    let user_repo = fin_sight_backend::repositories::UserRepository::new(
        fin_sight_backend::repositories::DbConnection::new(pool.clone())
    );
    let account_repo = fin_sight_backend::repositories::AccountRepository::new(
        fin_sight_backend::repositories::DbConnection::new(pool.clone())
    );
    let category_repo = fin_sight_backend::repositories::CategoryRepository::new(
        fin_sight_backend::repositories::DbConnection::new(pool.clone())
    );
    let transaction_repo = fin_sight_backend::repositories::TransactionRepository::new(
        fin_sight_backend::repositories::DbConnection::new(pool.clone())
    );

    let app_state = AppState {
        user_repo,
        account_repo,
        category_repo,
        transaction_repo,
    };

    let app = Router::new()
        .route("/api/users", post(|State(state): fin_sight_backend::axum::extract::State<AppState>, fin_sight_backend::axum::extract::Json(payload): fin_sight_backend::axum::extract::Json<CreateUser>| async {
            let user = state.user_repo.create(&payload).await.unwrap();
            fin_sight_backend::axum::response::Json(fin_sight_backend::models::ApiResponse::success(user))
        }))
        .with_state(app_state);

    let user_data = json!({
        "email": "test@example.com",
        "password": "password123"
    });

    let response = app
        .oneshot(Request::builder()
            .method("POST")
            .uri("/api/users")
            .header("content-type", "application/json")
            .body(Body::from(user_data.to_string()))
            .unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}