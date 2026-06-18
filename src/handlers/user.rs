use crate::entities::{user, user::Entity as User};
use crate::errors::api::ApiError;
use crate::state::AppState;
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::*;
use serde::{Deserialize, Serialize};

// Получить пользователя по ID
pub async fn get_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<Json<user::Model>, ApiError> {
    let user = User::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|e| ApiError::internal_bd(format!("Ошибка базы данных: {}", e)))?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(ApiError::user_not_found()),
    }
}

#[derive(Deserialize)]
pub struct CreateUserDto {
    name: String,
    email: String,
}

#[derive(Serialize)]
pub struct UserId {
    id: i32,
}

// Создать пользователя
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<UserId>), ApiError> {
    let new_user = user::ActiveModel {
        id: NotSet,
        name: Set(payload.name),
        email: Set(payload.email),
    };

    let inserted_user = new_user
        .insert(&state.db)
        .await
        .map_err(|e| ApiError::internal_bd(format!("Ошибка при создании пользователя: {}", e)))?;

    Ok((
        StatusCode::CREATED,
        Json(UserId {
            id: inserted_user.id,
        }),
    ))
}

pub async fn delete_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<(StatusCode, Json<UserId>), ApiError> {
    let user = User::delete_by_id(id)
        .exec_with_returning(&state.db)
        .await
        .map_err(|e| ApiError::internal_bd(format!("Ошибка удаления пользователя: {}", e)))?;

    match user {
        Some(user) => Ok((StatusCode::OK, Json(UserId { id: user.id }))),
        None => Err(ApiError::user_not_found()),
    }
}
