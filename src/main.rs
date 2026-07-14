use axum_app::database::{register_tables, state::AppState};
use axum_app::handlers::register_handlers;
use axum_app::log::init_logging;
use axum_app::misc::env_handle::{
    build_postgres_db_url, get_env_host_by_name, get_env_port_by_name, ENV_DB_HOST_NAME,
    ENV_DB_PORT_NAME, ENV_HOST_NAME, ENV_PORT_NAME, HOST_DEFAULT, PORT_DEFAULT,
};
use entities::sea_orm::Database;
use entities::sea_orm_migration::MigratorTrait;
use migration::Migrator;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let db_url = build_postgres_db_url().inspect_err(|er| {
        tracing::error!(
            "Can't build database url. Check env values: '{ENV_DB_HOST_NAME}' and '{ENV_DB_PORT_NAME}'. Error: {}",
            er
        );
    })?;
    let db = Database::connect(db_url).await.inspect_err(|er| {
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

    let app_host = get_env_host_by_name(ENV_HOST_NAME, HOST_DEFAULT).inspect_err(|er| {
        tracing::error!("env '{ENV_HOST_NAME}' should be IPv4 format: {}", er);
    })?;
    let app_port = get_env_port_by_name(ENV_PORT_NAME, PORT_DEFAULT).inspect_err(|er| {
        tracing::error!(
            "env '{ENV_PORT_NAME}' should be in range [0, 65535]: {}",
            er
        );
    })?;

    let addr = SocketAddr::from((app_host, app_port));
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
