use sea_orm::Database;
use sea_orm_migration::MigratorTrait;
use std::net::SocketAddr;

mod database;
mod errors;
mod handlers;

use database::{register_tables, state::AppState};
use handlers::register_handlers;
use migration::Migrator;

#[tokio::main]
async fn main() {
    let db_url = "postgres://user:user@localhost:5432/db-test";
    let db = Database::connect(db_url)
        .await
        .expect("Can't connect to database");

    if cfg!(debug_assertions) {
        register_tables(&db).await.unwrap();
    } else {
        println!("Release migration - start.");
        Migrator::up(&db, None)
            .await
            .expect("Error while migration");
        println!("Release migration - done.");
    }

    let state = AppState { db };

    let app = register_handlers(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server started on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
