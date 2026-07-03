use entities::sea_orm::Database;
use entities::sea_orm_migration::MigratorTrait;
use std::net::SocketAddr;

mod database;
mod errors;
mod handlers;
mod log;

use database::{register_tables, state::AppState};
use handlers::register_handlers;
use log::init_logging;
use migration::Migrator;

#[tokio::main]
async fn main() {
    init_logging();

    let db_url = "postgres://user:user@localhost:5432/db-test";
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    tracing::info!("Server started on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
