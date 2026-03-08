-- Migration to fix SQLite type compatibility
-- This migration ensures all data types are compatible with SQLite

-- Recreate users table with proper SQLite types
CREATE TABLE IF NOT EXISTS users_new (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    salt TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

INSERT OR IGNORE INTO users_new (id, email, password_hash, salt, created_at, updated_at)
SELECT id, email, password_hash, salt, created_at, updated_at FROM users;

DROP TABLE users;
ALTER TABLE users_new RENAME TO users;

-- Recreate accounts table with proper SQLite types
CREATE TABLE IF NOT EXISTS accounts_new (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    plaid_account_id TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    account_type TEXT NOT NULL,
    subtype TEXT,
    mask TEXT,
    current_balance REAL NOT NULL,
    available_balance REAL,
    iso_currency_code TEXT,
    unofficial_currency_code TEXT,
    plaid_access_token TEXT,
    plaid_item_id TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

INSERT OR IGNORE INTO accounts_new (id, user_id, plaid_account_id, name, account_type, subtype, mask, current_balance, available_balance, iso_currency_code, unofficial_currency_code, plaid_access_token, plaid_item_id, created_at, updated_at)
SELECT id, user_id, plaid_account_id, name, account_type, subtype, mask, current_balance, available_balance, iso_currency_code, unofficial_currency_code, plaid_access_token, plaid_item_id, created_at, updated_at FROM accounts;

DROP TABLE accounts;
ALTER TABLE accounts_new RENAME TO accounts;

-- Recreate transactions table with proper SQLite types
CREATE TABLE IF NOT EXISTS transactions_new (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL,
    plaid_transaction_id TEXT UNIQUE NOT NULL,
    category_id INTEGER,
    amount REAL NOT NULL,
    currency_code TEXT NOT NULL,
    date TEXT NOT NULL,
    description TEXT,
    merchant_name TEXT,
    payment_channel TEXT,
    pending INTEGER DEFAULT 0,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (account_id) REFERENCES accounts(id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

INSERT OR IGNORE INTO transactions_new (id, account_id, plaid_transaction_id, category_id, amount, currency_code, date, description, merchant_name, payment_channel, pending, created_at, updated_at)
SELECT id, account_id, plaid_transaction_id, category_id, amount, currency_code, date, description, merchant_name, payment_channel, pending, created_at, updated_at FROM transactions;

DROP TABLE transactions;
ALTER TABLE transactions_new RENAME TO transactions;

-- Ensure indexes exist
CREATE INDEX IF NOT EXISTS idx_transactions_date ON transactions(date);
CREATE INDEX IF NOT EXISTS idx_transactions_category ON transactions(category_id);
CREATE INDEX IF NOT EXISTS idx_transactions_account ON transactions(account_id);
CREATE INDEX IF NOT EXISTS idx_accounts_user_id ON accounts(user_id);
CREATE INDEX IF NOT EXISTS idx_accounts_plaid_item_id ON accounts(plaid_item_id);