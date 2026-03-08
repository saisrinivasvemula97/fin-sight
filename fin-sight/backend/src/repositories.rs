use sqlx::{SqlitePool, Row, Error as SqlxError};
use crate::models::*;

/// Database connection wrapper
#[derive(Clone)]
pub struct DbConnection {
    pool: SqlitePool,
}

impl DbConnection {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

// ─── User Repository ───────────────────────────────────────────────────────

#[derive(Clone)]
pub struct UserRepository {
    db: DbConnection,
}

impl UserRepository {
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, user: &CreateUser) -> Result<User, SqlxError> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().to_rfc3339();
        let updated_at = created_at.clone();

        sqlx::query(
            "INSERT INTO users (id, email, password_hash, salt, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&user.email)
        .bind(&user.password)
        .bind("salt_placeholder")
        .bind(&created_at)
        .bind(&updated_at)
        .execute(&self.db.pool)
        .await?;

        self.find_by_id(&id).await?
            .ok_or(SqlxError::RowNotFound)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, SqlxError> {
        let row = sqlx::query(
            "SELECT id, email, password_hash, salt, created_at, updated_at FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(User {
                id: row.try_get("id")?,
                email: row.try_get("email")?,
                password_hash: row.try_get("password_hash")?,
                salt: row.try_get("salt")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            })),
            None => Ok(None),
        }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>, SqlxError> {
        let row = sqlx::query(
            "SELECT id, email, password_hash, salt, created_at, updated_at FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(User {
                id: row.try_get("id")?,
                email: row.try_get("email")?,
                password_hash: row.try_get("password_hash")?,
                salt: row.try_get("salt")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            })),
            None => Ok(None),
        }
    }
}

// ─── Account Repository ────────────────────────────────────────────────────

#[derive(Clone)]
pub struct AccountRepository {
    db: DbConnection,
}

impl AccountRepository {
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, account: &CreateAccount) -> Result<Account, SqlxError> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().to_rfc3339();
        let updated_at = created_at.clone();

        sqlx::query(
            "INSERT INTO accounts (id, user_id, plaid_account_id, name, account_type, subtype, mask, current_balance, available_balance, iso_currency_code, unofficial_currency_code, plaid_access_token, plaid_item_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&account.user_id)
        .bind(&account.plaid_account_id)
        .bind(&account.name)
        .bind(&account.account_type)
        .bind(&account.subtype)
        .bind(&account.mask)
        .bind(account.current_balance)
        .bind(account.available_balance)
        .bind(&account.iso_currency_code)
        .bind(&account.unofficial_currency_code)
        .bind(&account.plaid_access_token)
        .bind(&account.plaid_item_id)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(&self.db.pool)
        .await?;

        self.find_by_plaid_account_id(&account.plaid_account_id).await?
            .ok_or(SqlxError::RowNotFound)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Account>, SqlxError> {
        let row = sqlx::query(
            "SELECT id, user_id, plaid_account_id, name, account_type, subtype, mask, current_balance, available_balance, iso_currency_code, unofficial_currency_code, plaid_access_token, plaid_item_id, created_at, updated_at FROM accounts WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_account(row)?)),
            None => Ok(None),
        }
    }

    pub async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Account>, SqlxError> {
        let rows = sqlx::query(
            "SELECT id, user_id, plaid_account_id, name, account_type, subtype, mask, current_balance, available_balance, iso_currency_code, unofficial_currency_code, plaid_access_token, plaid_item_id, created_at, updated_at FROM accounts WHERE user_id = ? ORDER BY name"
        )
        .bind(user_id)
        .fetch_all(&self.db.pool)
        .await?;

        let accounts = rows.into_iter()
            .map(|row| Self::row_to_account(row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(accounts)
    }

    pub async fn find_by_plaid_account_id(&self, plaid_account_id: &str) -> Result<Option<Account>, SqlxError> {
        let row = sqlx::query(
            "SELECT id, user_id, plaid_account_id, name, account_type, subtype, mask, current_balance, available_balance, iso_currency_code, unofficial_currency_code, plaid_access_token, plaid_item_id, created_at, updated_at FROM accounts WHERE plaid_account_id = ?"
        )
        .bind(plaid_account_id)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_account(row)?)),
            None => Ok(None),
        }
    }

    pub async fn find_by_plaid_item_id(&self, plaid_item_id: &str) -> Result<Vec<Account>, SqlxError> {
        let rows = sqlx::query(
            "SELECT id, user_id, plaid_account_id, name, account_type, subtype, mask, current_balance, available_balance, iso_currency_code, unofficial_currency_code, plaid_access_token, plaid_item_id, created_at, updated_at FROM accounts WHERE plaid_item_id = ?"
        )
        .bind(plaid_item_id)
        .fetch_all(&self.db.pool)
        .await?;

        let accounts = rows.into_iter()
            .map(|row| Self::row_to_account(row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(accounts)
    }

    pub async fn update_balance(&self, id: &str, current_balance: f64, available_balance: Option<f64>) -> Result<Account, SqlxError> {
        let updated_at = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE accounts SET current_balance = ?, available_balance = ?, updated_at = ? WHERE id = ?"
        )
        .bind(current_balance)
        .bind(available_balance)
        .bind(&updated_at)
        .bind(id)
        .execute(&self.db.pool)
        .await?;

        self.find_by_id(id).await?
            .ok_or(SqlxError::RowNotFound)
    }

    fn row_to_account(row: sqlx::sqlite::SqliteRow) -> Result<Account, SqlxError> {
        Ok(Account {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            plaid_account_id: row.try_get("plaid_account_id")?,
            name: row.try_get("name")?,
            account_type: row.try_get("account_type")?,
            subtype: row.try_get("subtype")?,
            mask: row.try_get("mask")?,
            current_balance: row.try_get("current_balance")?,
            available_balance: row.try_get("available_balance")?,
            iso_currency_code: row.try_get("iso_currency_code")?,
            unofficial_currency_code: row.try_get("unofficial_currency_code")?,
            plaid_access_token: row.try_get("plaid_access_token")?,
            plaid_item_id: row.try_get("plaid_item_id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

// ─── Category Repository ───────────────────────────────────────────────────

#[derive(Clone)]
pub struct CategoryRepository {
    db: DbConnection,
}

impl CategoryRepository {
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<Category>, SqlxError> {
        let rows = sqlx::query(
            "SELECT id, name, category_type, parent_id, is_custom, created_at FROM categories ORDER BY category_type, name"
        )
        .fetch_all(&self.db.pool)
        .await?;

        let categories = rows.into_iter()
            .map(|row| Self::row_to_category(row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Category>, SqlxError> {
        let row = sqlx::query(
            "SELECT id, name, category_type, parent_id, is_custom, created_at FROM categories WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_category(row)?)),
            None => Ok(None),
        }
    }

    pub async fn create(&self, category: &CreateCategory) -> Result<Category, SqlxError> {
        let created_at = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO categories (name, category_type, parent_id, is_custom, created_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&category.name)
        .bind(&category.category_type)
        .bind(category.parent_id)
        .bind(category.is_custom)
        .bind(&created_at)
        .execute(&self.db.pool)
        .await?;

        self.find_by_name(&category.name).await?
            .ok_or(SqlxError::RowNotFound)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Category>, SqlxError> {
        let row = sqlx::query(
            "SELECT id, name, category_type, parent_id, is_custom, created_at FROM categories WHERE name = ?"
        )
        .bind(name)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_category(row)?)),
            None => Ok(None),
        }
    }

    fn row_to_category(row: sqlx::sqlite::SqliteRow) -> Result<Category, SqlxError> {
        Ok(Category {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            category_type: row.try_get("category_type")?,
            parent_id: row.try_get("parent_id")?,
            is_custom: row.try_get("is_custom")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

// ─── Transaction Repository ────────────────────────────────────────────────

#[derive(Clone)]
pub struct TransactionRepository {
    db: DbConnection,
}

impl TransactionRepository {
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, txn: &CreateTransaction) -> Result<Transaction, SqlxError> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().to_rfc3339();
        let updated_at = created_at.clone();

        sqlx::query(
            "INSERT INTO transactions (id, account_id, plaid_transaction_id, category_id, amount, currency_code, date, description, merchant_name, payment_channel, pending, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&txn.account_id)
        .bind(&txn.plaid_transaction_id)
        .bind(txn.category_id)
        .bind(txn.amount)
        .bind(&txn.currency_code)
        .bind(&txn.date)
        .bind(&txn.description)
        .bind(&txn.merchant_name)
        .bind(&txn.payment_channel)
        .bind(txn.pending)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(&self.db.pool)
        .await?;

        self.find_by_plaid_transaction_id(&txn.plaid_transaction_id).await?
            .ok_or(SqlxError::RowNotFound)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Transaction>, SqlxError> {
        let row = sqlx::query(
            "SELECT id, account_id, category_id, amount, currency_code, date, description, merchant_name, payment_channel, pending, created_at, updated_at FROM transactions WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_transaction(row)?)),
            None => Ok(None),
        }
    }

    pub async fn find_by_account_id(&self, account_id: &str, limit: i64, offset: i64) -> Result<Vec<Transaction>, SqlxError> {
        let rows = sqlx::query(
            "SELECT id, account_id, category_id, amount, currency_code, date, description, merchant_name, payment_channel, pending, created_at, updated_at FROM transactions WHERE account_id = ? ORDER BY date DESC, created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(account_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db.pool)
        .await?;

        let transactions = rows.into_iter()
            .map(|row| Self::row_to_transaction(row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(transactions)
    }

    pub async fn find_by_user_id(&self, user_id: &str, limit: i64, offset: i64) -> Result<Vec<Transaction>, SqlxError> {
        let rows = sqlx::query(
            "SELECT t.id, t.account_id, t.category_id, t.amount, t.currency_code, t.date, t.description, t.merchant_name, t.payment_channel, t.pending, t.created_at, t.updated_at FROM transactions t JOIN accounts a ON t.account_id = a.id WHERE a.user_id = ? ORDER BY t.date DESC, t.created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db.pool)
        .await?;

        let transactions = rows.into_iter()
            .map(|row| Self::row_to_transaction(row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(transactions)
    }

    pub async fn update(&self, id: &str, update: &UpdateTransaction) -> Result<Transaction, SqlxError> {
        let updated_at = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE transactions SET category_id = COALESCE(?, category_id), description = COALESCE(?, description), updated_at = ? WHERE id = ?"
        )
        .bind(update.category_id)
        .bind(&update.description)
        .bind(&updated_at)
        .bind(id)
        .execute(&self.db.pool)
        .await?;

        self.find_by_id(id).await?
            .ok_or(SqlxError::RowNotFound)
    }

    pub async fn delete(&self, id: &str) -> Result<(), SqlxError> {
        sqlx::query("DELETE FROM transactions WHERE id = ?")
            .bind(id)
            .execute(&self.db.pool)
            .await?;
        Ok(())
    }

    pub async fn get_category_summary(&self, user_id: &str, start_date: Option<&str>, end_date: Option<&str>) -> Result<Vec<CategorySummary>, SqlxError> {
        let rows = sqlx::query(
            "SELECT c.id, c.name, COALESCE(SUM(t.amount), 0), COUNT(t.id) FROM categories c LEFT JOIN transactions t ON c.id = t.category_id LEFT JOIN accounts a ON t.account_id = a.id WHERE a.user_id = ? AND (? IS NULL OR t.date >= ?) AND (? IS NULL OR t.date <= ?) AND c.category_type = 'expense' GROUP BY c.id, c.name HAVING SUM(t.amount) > 0 ORDER BY SUM(t.amount) DESC"
        )
        .bind(user_id)
        .bind(start_date)
        .bind(start_date)
        .bind(end_date)
        .bind(end_date)
        .fetch_all(&self.db.pool)
        .await?;

        let summary = rows.into_iter()
            .map(|row| -> Result<CategorySummary, SqlxError> {
                Ok(CategorySummary {
                    category_id: row.try_get(0)?,
                    category_name: row.try_get(1)?,
                    total_amount: row.try_get(2)?,
                    transaction_count: row.try_get(3)?,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(summary)
    }

    pub async fn get_net_worth(&self, user_id: &str) -> Result<NetWorth, SqlxError> {
        let row = sqlx::query(
            "SELECT COALESCE(SUM(CASE WHEN a.account_type NOT IN ('credit', 'loan', 'mortgage') THEN a.current_balance ELSE 0 END), 0), COALESCE(SUM(CASE WHEN a.account_type IN ('credit', 'loan', 'mortgage') THEN a.current_balance ELSE 0 END), 0) FROM accounts a WHERE a.user_id = ?"
        )
        .bind(user_id)
        .fetch_one(&self.db.pool)
        .await?;

        let total_assets: f64 = row.try_get(0)?;
        let total_liabilities: f64 = row.try_get(1)?;
        let net_worth = total_assets - total_liabilities;

        Ok(NetWorth {
            total_assets,
            total_liabilities,
            net_worth,
        })
    }

    async fn find_by_plaid_transaction_id(&self, plaid_transaction_id: &str) -> Result<Option<Transaction>, SqlxError> {
        let row = sqlx::query(
            "SELECT id, account_id, category_id, amount, currency_code, date, description, merchant_name, payment_channel, pending, created_at, updated_at FROM transactions WHERE plaid_transaction_id = ?"
        )
        .bind(plaid_transaction_id)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(Self::row_to_transaction(row)?)),
            None => Ok(None),
        }
    }

    fn row_to_transaction(row: sqlx::sqlite::SqliteRow) -> Result<Transaction, SqlxError> {
        Ok(Transaction {
            id: row.try_get("id")?,
            account_id: row.try_get("account_id")?,
            category_id: row.try_get("category_id")?,
            amount: row.try_get("amount")?,
            currency_code: row.try_get("currency_code")?,
            date: row.try_get("date")?,
            description: row.try_get("description")?,
            merchant_name: row.try_get("merchant_name")?,
            payment_channel: row.try_get("payment_channel")?,
            pending: row.try_get("pending")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}