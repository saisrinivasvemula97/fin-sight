-- SQLite Schema for FinSight
-- This schema is optimized for SQLite compatibility with sqlx

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    salt TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- Create accounts table
CREATE TABLE IF NOT EXISTS accounts (
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

-- Create categories table
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    category_type TEXT NOT NULL CHECK (category_type IN ('expense', 'income', 'transfer')),
    parent_id INTEGER,
    is_custom INTEGER DEFAULT 0,
    created_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (parent_id) REFERENCES categories(id)
);

-- Create transactions table
CREATE TABLE IF NOT EXISTS transactions (
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

-- Create indexes for analytical queries
CREATE INDEX IF NOT EXISTS idx_transactions_date ON transactions(date);
CREATE INDEX IF NOT EXISTS idx_transactions_category ON transactions(category_id);
CREATE INDEX IF NOT EXISTS idx_transactions_account ON transactions(account_id);
CREATE INDEX IF NOT EXISTS idx_accounts_user_id ON accounts(user_id);
CREATE INDEX IF NOT EXISTS idx_accounts_plaid_item_id ON accounts(plaid_item_id);

-- Insert default categories
INSERT OR IGNORE INTO categories (name, category_type, parent_id, is_custom) VALUES
    -- Expense categories
    ('Food & Dining', 'expense', NULL, 0),
    ('Groceries', 'expense', (SELECT id FROM categories WHERE name = 'Food & Dining'), 0),
    ('Restaurants', 'expense', (SELECT id FROM categories WHERE name = 'Food & Dining'), 0),
    ('Transportation', 'expense', NULL, 0),
    ('Gas & Fuel', 'expense', (SELECT id FROM categories WHERE name = 'Transportation'), 0),
    ('Public Transit', 'expense', (SELECT id FROM categories WHERE name = 'Transportation'), 0),
    ('Entertainment', 'expense', NULL, 0),
    ('Movies & DVDs', 'expense', (SELECT id FROM categories WHERE name = 'Entertainment'), 0),
    ('Music', 'expense', (SELECT id FROM categories WHERE name = 'Entertainment'), 0),
    ('Shopping', 'expense', NULL, 0),
    ('Clothing', 'expense', (SELECT id FROM categories WHERE name = 'Shopping'), 0),
    ('Electronics', 'expense', (SELECT id FROM categories WHERE name = 'Shopping'), 0),
    ('Bills & Utilities', 'expense', NULL, 0),
    ('Electricity', 'expense', (SELECT id FROM categories WHERE name = 'Bills & Utilities'), 0),
    ('Water', 'expense', (SELECT id FROM categories WHERE name = 'Bills & Utilities'), 0),
    ('Rent', 'expense', (SELECT id FROM categories WHERE name = 'Bills & Utilities'), 0),
    ('Healthcare', 'expense', NULL, 0),
    ('Doctor', 'expense', (SELECT id FROM categories WHERE name = 'Healthcare'), 0),
    ('Pharmacy', 'expense', (SELECT id FROM categories WHERE name = 'Healthcare'), 0),
    ('Income', 'income', NULL, 0),
    ('Paycheck', 'income', (SELECT id FROM categories WHERE name = 'Income'), 0),
    ('Bonus', 'income', (SELECT id FROM categories WHERE name = 'Income'), 0),
    ('Transfer', 'transfer', NULL, 0),
    ('Transfer In', 'transfer', (SELECT id FROM categories WHERE name = 'Transfer'), 0),
    ('Transfer Out', 'transfer', (SELECT id FROM categories WHERE name = 'Transfer'), 0);