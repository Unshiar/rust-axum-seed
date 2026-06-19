use axum::{
    Router,
    routing::{delete, get, post},
};
use sea_orm::{Database, DbErr};
use std::net::SocketAddr;

mod errors;
mod handlers;
mod state;

use handlers::user::{create_user, delete_user, get_user};
use migration::MigrationTrait;
use migration::SchemaManager;
use migration::m20220101_000001_create_table;
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    // 1. Инициализируем подключение к БД через SeaORM
    let db_url = "postgres://user:user@localhost:5432/db-test";
    let db = Database::connect(db_url)
        .await
        .expect("Не удалось подключиться к базе данных");

    let schema_manager = SchemaManager::new(&db);
    // 2. Вызываем метод .up() конкретной миграции напрямую!
    m20220101_000001_create_table::Migration
        .up(&schema_manager)
        .await?;

    let state = AppState { db };

    // 2. Настраиваем маршруты и передаем в них состояние
    let app = Router::new()
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", delete(delete_user))
        .with_state(state);

    // 3. Запускаем сервер с помощью Tokio
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Сервер запущен на http://{}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
