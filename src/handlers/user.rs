use crate::database::state::AppState;
use crate::errors::api::ApiError;
use axum::{extract::State, http::StatusCode, Json};
use entities::sea_orm::*;
use entities::{user, user::Entity as User};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub async fn get_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<Json<user::Model>, ApiError> {
    let user = User::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|er| ApiError::internal_bd(&er))?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(ApiError::user_not_found()),
    }
}

pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<user::Model>>, ApiError> {
    let users = User::find()
        .all(&state.db)
        .await
        .map_err(|er| ApiError::internal_bd(&er))?;

    Ok(Json(users))
}

#[derive(Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 5))]
    name: String,
    #[validate(email)]
    email: String,
}

#[derive(Serialize)]
pub struct UserId {
    id: i32,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<UserId>), ApiError> {
    match payload.validate() {
        Ok(_) => (),
        Err(er) => Err(ApiError::invalid_create_user_data(&er))?,
    }

    let new_user = user::ActiveModel {
        id: NotSet,
        name: Set(payload.name),
        email: Set(payload.email),
    };

    let inserted_user = new_user
        .insert(&state.db)
        .await
        .map_err(|er| ApiError::internal_bd(&er))?;

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
        .map_err(|er| ApiError::internal_bd(&er))?;

    match user {
        Some(user) => Ok((StatusCode::OK, Json(UserId { id: user.id }))),
        None => Err(ApiError::user_not_found()),
    }
}
