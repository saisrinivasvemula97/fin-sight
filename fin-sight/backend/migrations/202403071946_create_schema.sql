-- DuckDB Schema for FinSight
-- This schema is optimized for analytical queries and DuckDB's columnar storage

-- Enable WAL mode for better concurrency
PRAGMA enable_profiling='json';
PRAGMA wal_autocheckpoint=0;
PRAGMA checkpoint_frequency=0;

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    salt VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create accounts table
CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    plaid_account_id VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    account_type VARCHAR(50) NOT NULL,
    subtype VARCHAR(50),
    mask VARCHAR(10),
    current_balance DOUBLE NOT NULL,
    available_balance DOUBLE,
    iso_currency_code VARCHAR(3),
    unofficial_currency_code VARCHAR(3),
    plaid_access_token VARCHAR(255),
    plaid_item_id VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create categories table
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    category_type VARCHAR(20) NOT NULL CHECK (category_type IN ('expense', 'income', 'transfer')),
    parent_id INTEGER REFERENCES categories(id),
    is_custom BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES accounts(id),
    plaid_transaction_id VARCHAR(255) UNIQUE NOT NULL,
    category_id INTEGER REFERENCES categories(id),
    amount DOUBLE NOT NULL,
    currency_code VARCHAR(3) NOT NULL,
    date DATE NOT NULL,
    description VARCHAR(500),
    merchant_name VARCHAR(255),
    payment_channel VARCHAR(50),
    pending BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for analytical queries
CREATE INDEX IF NOT EXISTS idx_transactions_date ON transactions(date);
CREATE INDEX IF NOT EXISTS idx_transactions_category ON transactions(category_id);
CREATE INDEX IF NOT EXISTS idx_transactions_account ON transactions(account_id);
CREATE INDEX IF NOT EXISTS idx_accounts_user_id ON accounts(user_id);
CREATE INDEX IF NOT EXISTS idx_accounts_plaid_item_id ON accounts(plaid_item_id);

-- Insert default categories
INSERT OR IGNORE INTO categories (name, category_type, parent_id, is_custom) VALUES
    -- Expense categories
    ('Food & Dining', 'expense', NULL, FALSE),
    ('Groceries', 'expense', (SELECT id FROM categories WHERE name = 'Food & Dining'), FALSE),
    ('Restaurants', 'expense', (SELECT id FROM categories WHERE name = 'Food & Dining'), FALSE),
    ('Transportation', 'expense', NULL, FALSE),
    ('Gas & Fuel', 'expense', (SELECT id FROM categories WHERE name = 'Transportation'), FALSE),
    ('Public Transit', 'expense', (SELECT id FROM categories WHERE name = 'Transportation'), FALSE),
    ('Entertainment', 'expense', NULL, FALSE),
    ('Movies & DVDs', 'expense', (SELECT id FROM categories WHERE name = 'Entertainment'), FALSE),
    ('Music', 'expense', (SELECT id FROM categories WHERE name = 'Entertainment'), FALSE),
    ('Shopping', 'expense', NULL, FALSE),
    ('Clothing', 'expense', (SELECT id FROM categories WHERE name = 'Shopping'), FALSE),
    ('Electronics', 'expense', (SELECT id FROM categories WHERE name = 'Shopping'), FALSE),
    ('Bills & Utilities', 'expense', NULL, FALSE),
    ('Electricity', 'expense', (SELECT id FROM categories WHERE name = 'Bills & Utilities'), FALSE),
    ('Water', 'expense', (SELECT id FROM categories WHERE name = 'Bills & Utilities'), FALSE),
    ('Rent', 'expense', (SELECT id FROM categories WHERE name = 'Bills & Utilities'), FALSE),
    ('Healthcare', 'expense', NULL, FALSE),
    ('Doctor', 'expense', (SELECT id FROM categories WHERE name = 'Healthcare'), FALSE),
    ('Pharmacy', 'expense', (SELECT id FROM categories WHERE name = 'Healthcare'), FALSE),
    ('Income', 'income', NULL, FALSE),
    ('Paycheck', 'income', (SELECT id FROM categories WHERE name = 'Income'), FALSE),
    ('Bonus', 'income', (SELECT id FROM categories WHERE name = 'Income'), FALSE),
    ('Transfer', 'transfer', NULL, FALSE),
    ('Transfer In', 'transfer', (SELECT id FROM categories WHERE name = 'Transfer'), FALSE),
    ('Transfer Out', 'transfer', (SELECT id FROM categories WHERE name = 'Transfer'), FALSE);