use serde::{Deserialize, Serialize};

// ─── User Models ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub salt: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub password: Option<String>,
}

// ─── Account Models ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub user_id: String,
    pub plaid_account_id: String,
    pub name: String,
    pub account_type: String,
    pub subtype: Option<String>,
    pub mask: Option<String>,
    pub current_balance: f64,
    pub available_balance: Option<f64>,
    pub iso_currency_code: Option<String>,
    pub unofficial_currency_code: Option<String>,
    pub plaid_access_token: String,
    pub plaid_item_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccount {
    pub user_id: String,
    pub plaid_account_id: String,
    pub name: String,
    pub account_type: String,
    pub subtype: Option<String>,
    pub mask: Option<String>,
    pub current_balance: f64,
    pub available_balance: Option<f64>,
    pub iso_currency_code: Option<String>,
    pub unofficial_currency_code: Option<String>,
    pub plaid_access_token: String,
    pub plaid_item_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccount {
    pub name: Option<String>,
    pub current_balance: Option<f64>,
    pub available_balance: Option<f64>,
}

// ─── Category Models ───────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub category_type: String,
    pub parent_id: Option<i32>,
    pub is_custom: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub category_type: String,
    pub parent_id: Option<i32>,
    pub is_custom: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub parent_id: Option<i32>,
    pub is_custom: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CategoryQuery {
    pub category_type: Option<String>,
    pub parent_id: Option<i32>,
}

// ─── Transaction Models ────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub account_id: String,
    pub category_id: Option<i32>,
    pub amount: f64,
    pub currency_code: String,
    pub date: String,
    pub description: String,
    pub merchant_name: Option<String>,
    pub payment_channel: Option<String>,
    pub pending: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransaction {
    pub account_id: String,
    pub plaid_transaction_id: String,
    pub category_id: Option<i32>,
    pub amount: f64,
    pub currency_code: String,
    pub date: String,
    pub description: String,
    pub merchant_name: Option<String>,
    pub payment_channel: Option<String>,
    pub pending: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTransaction {
    pub category_id: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransactionQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub category_id: Option<i32>,
    pub account_id: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ─── Analytics Models ──────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct CategorySummary {
    pub category_id: i32,
    pub category_name: String,
    pub total_amount: f64,
    pub transaction_count: i64,
}

#[derive(Debug, Serialize)]
pub struct NetWorth {
    pub total_assets: f64,
    pub total_liabilities: f64,
    pub net_worth: f64,
}

// ─── API Response Models ───────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            error: None,
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message: None,
            error: Some(message),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub success: bool,
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}