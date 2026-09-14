# rust-axum-seed App

A Rust REST API boilerplate-like application built with **Axum**, **SeaORM**, and **PostgreSQL**. Provides user management endpoints (as example) with database migration support.

Suitable for MVPs, startups, or anyone wanting to try using Rust as a REST API application.

## Features

- 🚀 High-performance async REST API using [Axum](https://github.com/tokio-rs/axum)
- 📝 Structured logging with [Tracing](https://tokio.rs/tokio/topics/tracing)
- 🔧 Environment-based configuration with sensible defaults
- 🐳 Docker Compose setup

## Tech Stack

- **Language**: Rust (2021 edition)
- **Web Framework**: [Axum 0.8](https://github.com/tokio-rs/axum)
- **Runtime**: [Tokio](https://tokio.rs/)
- **Database**: PostgreSQL / SQLite via [SeaORM 2.0](https://www.sea-ql.org/SeaORM/)
- **Database migrations**: via [SeaORM Migration](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/)
- **Serialization**: [Serde](https://serde.rs/)
- **Validation**: [validator](https://github.com/Keats/validator)
- **Api schemas**: [utoipa](https://github.com/juhaku/utoipa)
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
│   ├── schemas/             # OpenAPI/Swagger schema definitions
│   │   ├── mod.rs           # API schema module
│   │   ├── user.rs          # User API schema
│   │   └── health.rs        # Health check API schema
│   ├── handlers/
│   │   ├── mod.rs           # Handler registration & router setup
│   │   ├── user.rs          # User CRUD endpoints (create, list, get, delete)
│   │   └── health.rs        # Health check endpoint
│   ├── bin/
│   │   └── generate_schema.rs # OpenAPI schema generator binary
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
│   └── integration_test.rs # Integration test suite
├── .env.example            # Example of default environment
├── Dockerfile.debug        # Dockerfile for debug build
├── docker-build-debug.sh   # script for debug building an application in a container
├── docker-compose-debug.yml  # axum-app and PostgreSQL services together
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

### 2. Build axum-app application. This will build the application and make an image for it.

```bash
./docker-build-debug.sh
```

### 3. Start axum-app application and PostgreSQL (Docker)

```bash
docker compose -f docker-compose-debug.yml up -d
```

This starts a PostgreSQL instance in container on `0.0.0.0:5432` with credentials:
- **Username**: `user`
- **Password**: `user`
- **Database**: `db-test`

And axum-app application in container on `0.0.0.0:8080`

### 4. For axum-app application you can configure environment (Optional)

Create a `.env` file or use `.env.example`:

```
# Server Configuration
SERVER_IP=0.0.0.0
SERVER_PORT=8080

# Database Configuration
DATABASE_USER=user
DATABASE_PASSWORD=user
DATABASE_NAME=db-test
DATABASE_HOST=postgres
DATABASE_PORT=5432

# Logging Configuration
LOG_LEVEL=info

# Notes:
# - Copy this file to `.env` and adjust values for your environment.
# - For production, manage secrets using your secret manager or CI.
```

**Default values** (if env vars not set):

| Variable            | Default     | Description                |
|---------------------|-------------|----------------------------|
| `SERVER_IP`         | `0.0.0.0`   | Server bind address (IPv4) |
| `SERVER_PORT`       | `8080`      | Server port                |
| `DATABASE_USER`     | `user`      | Database user              |
| `DATABASE_PASSWORD` | `user`      | Database password          |
| `DATABASE_NAME`     | `db-test`   | Database name              |
| `DATABASE_HOST`     | `postgres`  | Database server address    |
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
pub const DB_HOST_DEFAULT: &str = "postgres";
pub const ENV_DB_PORT_NAME: &str = "DATABASE_PORT";
pub const DB_PORT_DEFAULT: u16 = 5432;
pub const ENV_SERVER_IP_NAME: &str = "SERVER_IP";
pub const SERVER_IP_DEFAULT: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
pub const ENV_SERVER_PORT_NAME: &str = "SERVER_PORT";
pub const SERVER_PORT_DEFAULT: u16 = 8080;
```
- All environment variables are optional and fall back to defaults if not set (with warnings in logs).

### 5. Wait until all containers is up and see axum-app logs

```bash
docker container logs app
```

Possible output:
```
2026-09-07T14:51:23.298141Z  INFO ThreadId(01) axum_app: Starting server
2026-09-07T14:51:23.298643Z  WARN ThreadId(01) axum_app::misc::env_handle: env 'DATABASE_PORT' is not set, using default
2026-09-07T14:51:23.298908Z  WARN ThreadId(01) axum_app::misc::env_handle: env 'DATABASE_USER' is not set, using default
2026-09-07T14:51:23.298953Z  WARN ThreadId(01) axum_app::misc::env_handle: env 'DATABASE_PASSWORD' is not set, using default
2026-09-07T14:51:23.298974Z  WARN ThreadId(01) axum_app::misc::env_handle: env 'DATABASE_NAME' is not set, using default
2026-09-07T14:51:23.385760Z  INFO ThreadId(01) axum_app::database: Creating/updating database tables
2026-09-07T14:51:23.396314Z  INFO ThreadId(01) entities: [DB] Table 'users' does not exists. Creating...
2026-09-07T14:51:23.489746Z  INFO ThreadId(01) entities: [DB] Done.
2026-09-07T14:51:23.493828Z  INFO ThreadId(01) axum_app: Server started on http://0.0.0.0:8080
```

### 6. API Endpoints example

#### User Management

| Method   | Endpoint     | Description       |
|----------|--------------|-------------------|
| `POST`   | `/user`      | Create a new user |
| `GET`    | `/user/{id}` | Get user by ID    |
| `GET`    | `/users`     | List all users    |
| `DELETE` | `/user/{id}` | Delete user by ID |

Examples (curl) - you run it on your local pc:

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

## API Schema & Swagger

The application provides OpenAPI/Swagger documentation for interactive API exploration.

### 1. Built-in Swagger UI (Debug Mode)

When running in **debug mode** (development), a built-in Swagger UI is available.

It's available on:
```
http://127.0.0.1:8080/swagger-ui
```

This provides an interactive interface to explore and test all API endpoints with automatic request/response documentation.

**Note:** The built-in Swagger UI is only available in debug builds. For production environments, use the generated OpenAPI schema with an external Swagger/OpenAPI viewer (e.g., Nginx proxy + Swagger UI).

### 2. Generate OpenAPI Schema

To generate the OpenAPI specification as a JSON file:

```bash
cargo run --bin generate_schema
```

This will create/update the `openapi.json` file in the project root, which contains the complete API specification compatible with any OpenAPI viewer or code generator.

## Logging

Using **tracing** and **tracing-subscriber** for structured logging.

The env var `LOG_LEVEL` can be used for global log level of all modules. Possible values are: `error`, `warn`, `info`, `debug`, `trace`.

Default value is `info`, if env var `LOG_LEVEL` is not set or invalid.

You can change the name of env var and hardcode the logging level for a specific module, see module `src/log/mod.rs`

## Database Migrations

Migrations are managed via SeaORM Migration:

- **Debug builds**: Tables are auto-created. No error occurs if a table already exists. (development convenience)
- **Release builds**: Migrations are applied from the `migration` workspace member

Add new migrations:

```bash
cd migration
sea-orm-cli migrate add <migration_name>
```

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

### Migration errors in release mode

- Ensure `migration/` workspace member is properly configured
- Run `cargo build --release` to rebuild

## How to get a clean app (without current user logic)

It's pretty simple. Follow the steps below:

1. Force remove `handlers/user.rs` file
2. Force remove `errors/user.rs` file
3. Force remove `entities/src/user.rs` file
4. Force remove `schemas/user.rs` file
5. Edit `handlers/mod.rs` file:
   - remove user routes in `register_handlers(state: AppState) -> Router` function:
   ```
        // User routes
        .route("/user", post(create_user))
        .route("/user/{id}", get(get_user))
        .route("/user/{id}", delete(delete_user))
        .route("/users", get(get_users))
   ```
6. Edit `entities/src/lib.rs` file:
   - edit `get_all_tables()` function, it should return `vec![]`
7. Edit `schemas/mod.rs` file:
   - edit `get_all_tables()` function, remove line `main_api.merge(UserApi::openapi());`
8. Run `cargo clippy` command. Fix all import errors and warnings (just remove them).
9. Edit `tests/integration_test.rs` file:
   - add `#[ignore]` to `test_migrator_up_after_register_tables()` test, like this:
   ```
    #[ignore]
    #[tokio::test]
    async fn test_migrator_up_after_register_tables() {
   ```
   Or add `#[ignore]` to all tests, if you want to modify them in the future. Or remove
   all of them, if they have become irrelevant.
10. (Optional) Comment or remove unnecessary error codes in `errors/codes.rs` file:
   ```
   pub enum ApiErrorCodes {
   UserNotFound = 3001,
   InvalidCreateUserData = 4002,
   DatabaseInternalError = 5001,
   }
   ```
11. Commands`cargo clippy` and `cargo test` should not produce errors.

Now you can write your own entities, endpoints and handlers, errors. Rewrite init migration if needed.

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
