# Project: FinSight – Modern Expense Tracker

## 1. Problem Statement

Managing personal finances across multiple bank accounts and credit cards is fragmented and manual. **FinSight** is a high-performance, memory-safe full-stack application built with **Rust**. It aggregates financial data via the Plaid API, provides automated transaction categorization, and offers a unified view of a user's financial health through a relational database.

## 2. SMART Goals

* **Specific:** Develop a backend API using **Rust (Axum)** and **PostgreSQL (sqlx)** that integrates with the Plaid API to fetch and store financial transactions.
* **Measurable:** Successfully implement at least **5 core API endpoints** (Auth, Link Account, Fetch Transactions, Category Summary, and Net Worth) that pass integration tests.
* **Achievable:** Focus on the "Backend-First" approach to master Rust's ownership model and SQL database migrations before moving to the React/Next.js frontend.
* **Relevant:** This project serves as a deep dive into systems programming (Rust) and relational data modeling (PostgreSQL), which are critical for high-reliability fintech applications.
* **Time-bound:** Establish the PostgreSQL schema and a running "Hello World" Axum server within **1 week**; complete the Plaid integration logic within **3 weeks**.

---

## 3. Core Features (The Roadmap)

### Phase 1: The Rust & Postgres Foundation

* **Async Server:** Set up an **Axum** web server with **Tokio** as the runtime.
* **Database Migrations:** Use `sqlx-cli` to manage version-controlled PostgreSQL schema changes.
* **Type-Safe CRUD:** Create Rust structs that map directly to database rows using `Serde` for JSON serialization.

### Phase 2: Financial Integration

* **Plaid Implementation:** Handle the OAuth-like "Link" flow to exchange public tokens for permanent access tokens.
* **Transaction Sync:** A background process (or endpoint) that pulls the last 30 days of data and performs "Upserts" (Update or Insert) to prevent duplicate transactions.

### Phase 3: Frontend Dashboard

* **State Management:** Fetch data from the Rust API and display it using React.
* **Financial Insights:** Generate a "Monthly Spend" breakdown using SQL aggregation functions (`SUM`, `GROUP BY`).

---

## 4. Technical Stack

* **Backend Language:** Rust (Stable)
* **Web Framework:** Axum
* **Database:** PostgreSQL
* **SQL Toolkit:** sqlx (for compile-time checked SQL)
* **Authentication:** JWT (Json Web Tokens) with `argon2` for password hashing.
* **Frontend:** Next.js (TypeScript) & Tailwind CSS.

---

## 5. Database Schema (PostgreSQL)

| Entity | Primary Key | Foreign Key | Purpose |
| --- | --- | --- | --- |
| **Users** | `uuid` | - | User credentials and settings. |
| **Accounts** | `uuid` | `user_id` | Stores bank details (Checking, Credit, etc). |
| **Transactions** | `uuid` | `account_id` | Individual spending records. |
| **Categories** | `id` | - | System-wide and custom labels. |

---

## 6. Implementation Challenges (Learning Focus)

* **Ownership & Borrowing:** Navigating Rust’s strict compiler rules when passing database connection pools between functions.
* **Error Handling:** Using the `Result` type effectively to handle API or Database failures without the app crashing.
* **Async/Await:** Managing asynchronous data fetching from external APIs while keeping the database connection efficient.