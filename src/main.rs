use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::Database;
use std::net::SocketAddr;

mod entities;
mod errors;
mod handlers;
mod state;
pub mod utils;

use handlers::user::{create_user, get_user};
use state::AppState;

#[tokio::main]
async fn main() {
    // 1. Инициализируем подключение к БД через SeaORM
    let db_url = "postgres://user:user@localhost:5432/db-test";
    let db = Database::connect(db_url)
        .await
        .expect("Не удалось подключиться к базе данных");

    let state = AppState { db };

    // 2. Настраиваем маршруты и передаем в них состояние
    let app = Router::new()
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user))
        .with_state(state);

    // 3. Запускаем сервер с помощью Tokio
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Сервер запущен на http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
