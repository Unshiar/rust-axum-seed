# rust-axum-seed App

A Rust REST API boilerplate-like application built with **Axum**, **SeaORM**, and **PostgreSQL**. Provides user management endpoints (as example) with database migration support.

Suitable for MVPs, startups, or anyone wanting to try using Rust as a REST API application.

## Features

- 🚀 High-performance async REST API using [Axum](https://github.com/tokio-rs/axum)
- 🗄️ Database ORM with [SeaORM](https://www.sea-ql.org/SeaORM/) supporting PostgreSQL and SQLite
- 🔄 Automatic database migrations with [SeaORM Migration](https://www.sea-ql.org/SeaORM/)
- 📝 Structured logging with [Tracing](https://tokio.rs/tokio/topics/tracing)
- 🔧 Environment-based configuration with sensible defaults
- 🐳 Docker Compose setup for PostgreSQL

## Tech Stack

- **Language**: Rust (2021 edition)
- **Web Framework**: [Axum 0.8](https://github.com/tokio-rs/axum)
- **Runtime**: [Tokio](https://tokio.rs/)
- **Database**: PostgreSQL / SQLite via [SeaORM 2.0](https://www.sea-ql.org/SeaORM/)
- **Serialization**: [Serde](https://serde.rs/)
- **Logging**: [Tracing & Tracing-Subscriber](https://tokio.rs/tokio/topics/tracing)

## Project Structure

### Workspace Layout

This is a **Cargo workspace** with three members:

```
.
├── axum-app (main application)
│   └── depends on: entities, migration
├── entities (shared crate)
│   └── used by: axum-app, migration
└── migration (migrations crate)
    └── depends on: entities
```

### Directory Structure

```
.
├── src/                     # Main application (axum-app)
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library exports
│   ├── handlers/
│   │   ├── mod.rs           # Handler registration & router setup
│   │   ├── user.rs          # User CRUD endpoints (create, list, get, delete)
│   │   └── health.rs        # Health check endpoint
│   ├── database/
│   │   ├── mod.rs           # Database utilities
│   │   └── state.rs         # App state with DB connection
│   ├── errors/
│   │   ├── mod.rs           # Error types
│   │   ├── api.rs           # API error responses
│   │   ├── internal.rs      # Internal errors
│   │   ├── user.rs          # User domain errors
│   │   └── codes.rs         # Error codes
│   ├── misc/
│   │   ├── mod.rs           # Misc helpers and re-exports
│   │   ├── env_handle.rs    # Environment variable handling & parsing
│   │   ├── config.rs        # Application configuration helpers
│   │   └── signals.rs       # Graceful shutdown & signal handling
│   └── log/
│       └── mod.rs           # Logging initialization
├── entities/                # Shared ORM entities library
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs          # Entity library & prelude
│       └── user.rs         # User entity definitions
├── migration/              # Database migrations runner
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       ├── main.rs         # Migration CLI entry point
│       ├── lib.rs          # Migration library
│       └── m20260624_074014_initial_schema.rs  # Initial schema migration
├── tests/                  # Integration tests
├── docker-compose.yml      # PostgreSQL service
└── Cargo.toml             # Workspace configuration
```

**Note:** The `entities` crate is a **shared library** used by both the main `axum-app` and the `migration` crate, enabling consistent entity definitions across the application and migrations.



## Prerequisites

- Rust 1.96+ ([Install](https://rustup.rs/))
- PostgreSQL 18+ or SQLite 3+
- Docker & Docker Compose (optional, for containerized PostgreSQL)

## Getting Started

### 1. Clone and Setup

```bash
git clone <repository-url>
cd rust-axum-seed
```

### 2. Start PostgreSQL (Docker)

```bash
docker-compose up -d
```

This starts a PostgreSQL instance on `127.0.0.1:5432` with credentials:
- **Username**: `user`
- **Password**: `user`
- **Database**: `db-test`

### 3. Configure Environment (Optional)

Create a `.env` file or use `.env.example`:

```
# Server Configuration
HOST=127.0.0.1
PORT=8080

# Database Configuration
DATABASE_USER=user
DATABASE_PASSWORD=user
DATABASE_NAME=db-test
DATABASE_HOST=127.0.0.1
DATABASE_PORT=5432

# Notes:
# - Copy this file to `.env` and adjust values for your environment.
# - For production, manage secrets using your secret manager or CI.
```

**Default values** (if env vars not set):

| Variable            | Default     | Description                |
|---------------------|-------------|----------------------------|
| `HOST`              | `127.0.0.1` | Server bind address (IPv4) |
| `PORT`              | `8080`      | Server port                |
| `DATABASE_USER`     | `user`      | Database user              |
| `DATABASE_PASSWORD` | `user`      | Database password          |
| `DATABASE_NAME`     | `db-test`   | Database name              |
| `DATABASE_HOST`     | `127.0.0.1` | Database server address    |
| `DATABASE_PORT`     | `5432`      | Database server port       |

You can change the names and values of default environment variables, see module `src/misc/env_handle.rs`

```
pub const ENV_DB_USER_NAME: &str = "DATABASE_USER";
pub const DB_USER_DEFAULT: &str = "user";
pub const ENV_DB_PASSWORD_NAME: &str = "DATABASE_PASSWORD";
pub const DB_PASSWORD_DEFAULT: &str = "user";
pub const ENV_DB_NAME_NAME: &str = "DATABASE_NAME";
pub const DB_NAME_DEFAULT: &str = "db-test";
pub const ENV_DB_HOST_NAME: &str = "DATABASE_HOST";
pub const DB_HOST_DEFAULT: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
pub const ENV_DB_PORT_NAME: &str = "DATABASE_PORT";
pub const DB_PORT_DEFAULT: u16 = 5432;
pub const ENV_HOST_NAME: &str = "HOST";
pub const HOST_DEFAULT: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
pub const ENV_PORT_NAME: &str = "PORT";
pub const PORT_DEFAULT: u16 = 8080;
...
```
- All environment variables are optional and fall back to defaults if not set (with warnings in logs).

### 4. Run the Application

```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`

### 5. API Endpoints example

#### User Management

| Method   | Endpoint     | Description       |
|----------|--------------|-------------------|
| `POST`   | `/user`      | Create a new user |
| `GET`    | `/user/{id}` | Get user by ID    |
| `GET`    | `/users`     | List all users    |
| `DELETE` | `/user/{id}` | Delete user by ID |

Examples (curl):

```bash
curl -X POST http://127.0.0.1:8080/user -H "Content-Type: application/json" -d '{"name": "User_1", "email": "user_1@test.com"}'
# Expected response:  {"id":1}
curl -X POST http://127.0.0.1:8080/user -H "Content-Type: application/json" -d '{"name": "User_2", "email": "user_2@test.com"}'
# Expected response:  {"id":2}

curl -X GET http://127.0.0.1:8080/user/1
# Expected response: {"id":1,"name":"User_1","email":"user_1@test.com"}

curl -X GET http://127.0.0.1:8080/users
# Expected response: [{"id":1,"name":"User_1","email":"user_1@test.com"},{"id":2,"name":"User_2","email":"user_2@test.com"}]

curl -X DELETE http://127.0.0.1:8080/user/1
# Expected response: {"id":1}
```

#### Health

| Method | Endpoint  | Description                   |
|--------|-----------|-------------------------------|
| `GET`  | `/health` | Returns service health/status |

Example (curl):

```bash
curl -X GET http://127.0.0.1:8080/health
# Expected response: {"status":"ok"}
```

## Database Migrations

Migrations are managed via SeaORM Migration:

- **Debug builds**: Tables are auto-created. No error occurs if a table already exists. (development convenience)
- **Release builds**: Migrations are applied from the `migration` workspace member

Add new migrations:

```bash
cd migration
sea-orm-cli migrate add <migration_name>
```

## Building for Production

```bash
cargo build --release
```

The binary will be available at `target/release/axum-app`

## Development

### 1. Add models(entities) and endpoints with handlers

- Add new model to `entities/src/<entity_name>.rs` as module
- Add new handler to `src/handlers/<entity_name>.rs` as module
- Register the new route(s) in the `src/handlers/mod.rs` file in the `register_handlers()` function

To create a new table in the database for your entity:

- Register it in the `get_all_tables()` function in the `entities/src/lib.rs` file
```
pub fn get_all_tables() -> Vec<&'static (dyn ManageSchema + Sync)> {
    vec![&user::Entity]
}
```

This is enough for debug development.

⚠️**Important:** For release development, don't forget write migration(s).

### 2. Format and lint

- Format Code

```bash
cargo fmt
```

- Lint Code - axum-app

```bash
cargo clippy -- -D warnings
```

- Lint Code - unit tests

```bash
cargo clippy --tests -- -D warnings
```

- Lint Code - integration tests

```bash
cargo clippy --test integration_test -- -D warnings
```

### 3. Run Tests

- Running unit tests locally

```bash
cargo test --lib
```

- Running integration tests locally

The project includes lightweight integration tests that prefer SQLite/in-memory where possible.

```bash
cargo test --test integration_test
```

## Troubleshooting

### Can't connect to database

- Ensure PostgreSQL is running: `docker-compose ps`
- Check connection env vars match your setup
- Verify port 5432 is accessible

### Migration errors in release mode

- Ensure `migration/` workspace member is properly configured
- Run `cargo build --release` to rebuild

### Port already in use

- Change the `PORT` env var: `PORT=8000 cargo run`

## License

MIT License - see [LICENSE](./LICENSE) file for details

## Contributing

### Pull Requests

- Keep PRs focused and reasonably sized
- Ensure all tests pass and clippy is happy
- Update documentation as needed

### Code Style

- Follow Rust conventions and idioms
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions small and focused

### Questions?

Feel free to open an issue or discussion if you have questions!
