use sea_orm::Database;
use std::net::SocketAddr;

mod database;
mod errors;
mod handlers;

use crate::database::register_tables;
use crate::handlers::register_handlers;
use database::state::AppState;

#[tokio::main]
async fn main() {
    // 1. Инициализируем подключение к БД через SeaORM
    let db_url = "postgres://user:user@localhost:5432/db-test";
    let db = Database::connect(db_url)
        .await
        .expect("Не удалось подключиться к базе данных");

    // let schema_manager = SchemaManager::new(&db);
    // // 2. Вызываем метод .up() конкретной миграции напрямую!
    // m20220101_000001_create_table::Migration
    //     .up(&schema_manager)
    //     .await?;

    if cfg!(debug_assertions) {
        register_tables(&db).await.unwrap();
    } else {
        println!("Release migration");
    }
    let state = AppState { db };

    // 2. Настраиваем маршруты и передаем в них состояние
    let app = register_handlers(state);

    // 3. Запускаем сервер с помощью Tokio
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Сервер запущен на http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
