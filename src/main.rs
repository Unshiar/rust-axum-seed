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
async fn main() {
    init_logging();

    let db_url = get_env_db_url();
    let db = Database::connect(db_url)
        .await
        .expect("Can't connect to database");

    if cfg!(debug_assertions) {
        register_tables(&db).await.unwrap();
    } else {
        match Migrator::up(&db, None).await {
            Ok(_) => {
                tracing::info!("Successfully applied migrations");
            }
            Err(e) => {
                tracing::error!("Failed to apply migrations: {:?}", e);
                std::process::exit(1);
            }
        }
    }

    let state = AppState { db };
    let app = register_handlers(state);

    let host = get_env_host();
    let port = get_env_port();

    let addr = SocketAddr::from((host, port));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Server started on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
