# FinSight Backend

A comprehensive personal finance management backend API built with Rust, Axum, and SQLx.

## Features

- **User Management**: Create and manage user accounts
- **Account Management**: Link and manage financial accounts
- **Transaction Tracking**: Record and categorize transactions
- **Category Management**: Custom and predefined expense/income categories
- **Analytics**: Net worth tracking and category-based spending analysis
- **Database**: SQLite with automatic migrations
- **API**: RESTful API with proper error handling and CORS support

## Tech Stack

- **Rust** - Backend language
- **Axum** - Web framework
- **SQLx** - Async SQL toolkit
- **SQLite** - Database
- **Tokio** - Async runtime
- **Tracing** - Logging and observability
- **dotenvy** - Environment variable management

## Project Structure

```
fin-sight/
├── backend/
│   ├── src/
│   │   ├── main.rs          # Application entry point
│   │   ├── database.rs      # Database connection and setup
│   │   ├── models.rs        # Data models and API responses
│   │   ├── repositories.rs  # Database access layer
│   │   ├── routes.rs        # API route definitions
│   │   └── simple_routes.rs # Simple route handlers
│   ├── migrations/          # Database schema migrations
│   ├── tests/               # Integration tests
│   └── Cargo.toml          # Dependencies and configuration
├── frontend/               # (Future frontend implementation)
└── README.md              # This file
```

## Installation

### Prerequisites

- Rust 1.70+
- Cargo

### Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd FinSight/fin-sight
```

2. Navigate to the backend directory:
```bash
cd backend
```

3. Install dependencies:
```bash
cargo build
```

## Running the Application

### Development

Start the development server:
```bash
cargo run
```

The server will start on `http://localhost:3000` by default.

### Environment Variables

Create a `.env` file in the backend directory:

```env
PORT=3000
DATABASE_URL=sqlite:data/fin_sight.db
```

## API Endpoints

### Health Check
- `GET /api/health` - Health check endpoint

### Users
- `POST /api/users` - Create a new user
- `GET /api/users/{id}` - Get user by ID

### Accounts
- `POST /api/accounts` - Create a new account
- `GET /api/accounts/{id}` - Get account by ID
- `GET /api/accounts/user/{user_id}` - Get accounts by user
- `POST /api/accounts/{id}/balance` - Update account balance

### Categories
- `GET /api/categories` - Get all categories
- `GET /api/categories/{id}` - Get category by ID
- `POST /api/categories` - Create a new category

### Transactions
- `POST /api/transactions` - Create a new transaction
- `GET /api/transactions/{id}` - Get transaction by ID
- `GET /api/transactions/account/{account_id}` - Get transactions by account
- `GET /api/transactions/user/{user_id}` - Get transactions by user
- `POST /api/transactions/{id}` - Update transaction
- `DELETE /api/transactions/{id}` - Delete transaction

### Analytics
- `GET /api/analytics/category-summary` - Get category spending summary
- `GET /api/analytics/net-worth/{user_id}` - Get user's net worth

## Database Schema

The application uses SQLite with the following tables:

- `users` - User accounts
- `accounts` - Financial accounts
- `categories` - Transaction categories
- `transactions` - Financial transactions

Database migrations are automatically applied on startup.

## Testing

Run the test suite:
```bash
cargo test
```

## Development

### Code Style
- Follow Rust standard formatting with `rustfmt`
- Use `clippy` for linting
- Follow conventional commit messages

### Adding New Features
1. Add database migrations in `migrations/`
2. Update models in `src/models.rs`
3. Implement repository methods in `src/repositories.rs`
4. Add API endpoints in `src/main.rs`
5. Write tests in `tests/`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

This project is licensed under the MIT License.

## Future Enhancements

- Frontend React application
- Authentication and authorization
- Plaid integration for account linking
- Advanced analytics and reporting
- Budget management
- Mobile application
- Email notifications