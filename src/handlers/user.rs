use crate::entities::user::CreateUserDto;
use crate::entities::{user, user::Entity as User};
use crate::errors::ApiError;
use crate::state::AppState;
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::*;

// Получить пользователя по ID
pub async fn get_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<Json<user::Model>, ApiError> {
    let user = User::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| ApiError::internal(format!("Ошибка базы данных: {}", e)))?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(ApiError::not_found(format!(
            "Пользователь с ID {} не найден",
            id
        ))),
    }
}

// Создать пользователя
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<user::UserId>), ApiError> {
    let new_user = user::ActiveModel {
        id: NotSet,
        name: Set(payload.name),
        email: Set(payload.email),
    };

    let inserted_user = new_user
        .insert(&state.db)
        .await
        .map_err(|e| ApiError::internal(format!("Ошибка при создании пользователя: {}", e)))?;

    Ok((StatusCode::CREATED, Json(user::UserId { id: inserted_user.id })))
}
