use entities::sea_orm::Database;
use entities::sea_orm_migration::MigratorTrait;
use std::net::SocketAddr;

mod constants;
mod database;
mod env_handle;
mod errors;
mod handlers;
mod log;

use database::{register_tables, state::AppState};
use env_handle::{get_env_db_url, get_env_host, get_env_port};
use handlers::register_handlers;
use log::init_logging;
use migration::Migrator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let db = Database::connect(get_env_db_url())
        .await
        .inspect_err(|er| {
            tracing::error!("Can't connect to database: {}", er);
        })?;

    if cfg!(debug_assertions) {
        register_tables(&db).await.inspect_err(|er| {
            tracing::error!("Can't register tables: {}", er);
        })?;
    } else {
        Migrator::up(&db, None)
            .await
            .inspect_err(|_| tracing::error!("Failed to apply migrations"))?;
        tracing::info!("Successfully applied migrations");
    }

    let state = AppState { db };
    let app = register_handlers(state);

    let addr = SocketAddr::from((get_env_host()?, get_env_port()?));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .inspect_err(|er| {
            tracing::error!("Failed to bind to {}: {}", addr, er);
        })?;
    tracing::info!("Server started on http://{}", addr);

    axum::serve(listener, app).await.inspect_err(|er| {
        tracing::error!("Server startup error: {}", er);
    })?;

    Ok(())
}
