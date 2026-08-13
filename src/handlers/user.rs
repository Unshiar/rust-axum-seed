use crate::database::state::AppState;
use crate::errors::api::ApiError;
use axum::{extract::State, http::StatusCode, Json};
use entities::sea_orm::*;
use entities::user::Model;
use entities::{user, user::Entity as User};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/user/{id}",
    params(
        ("id" = i32, Path, description = "User unique identifier")
    ),
    responses(
        (status = 200, description = "User successfully found", body = Model),
        (status = 404, description = ApiError::user_not_found().message(), body = ApiError, example =  json!(ApiError::user_not_found())),
        (status = 500, description = ApiError::internal_bd() .message(), body = ApiError, example =  json!(ApiError::internal_bd())),
    ),
    tag = "Users",
)]
pub async fn get_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<Json<user::Model>, ApiError> {
    let user = User::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|er| ApiError::internal_bd().add_details(serde_json::json!(er.to_string())))?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(ApiError::user_not_found()),
    }
}

pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<user::Model>>, ApiError> {
    let users = User::find()
        .all(&state.db)
        .await
        .map_err(|er| ApiError::internal_bd().add_details(serde_json::json!(er.to_string())))?;

    Ok(Json(users))
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateUserDto {
    #[validate(length(min = 5))]
    #[schema(example = "UserName")]
    name: String,
    #[validate(email)]
    #[schema(example = "user@test.com")]
    email: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserId {
    #[schema(example = 1)]
    pub id: i32,
}
#[utoipa::path(
    post,
    path = "/user",
    request_body(description = "User create data",
        content(
        (CreateUserDto = "application/json"),
        )
    ),
    responses(
        (status = 201, description = "User successfully created", body = UserId),
        (status = 400, description = ApiError::invalid_create_user_data().message(), body = ApiError, example =  json!(ApiError::invalid_create_user_data())),
        (status = 500, description = ApiError::internal_bd() .message(), body = ApiError, example =  json!(ApiError::internal_bd())),
    ),
    tag = "Users",
)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<UserId>), ApiError> {
    payload
        .validate()
        .map_err(|er| ApiError::invalid_create_user_data().add_details(serde_json::json!(er)))?;

    let new_user = user::ActiveModel {
        id: NotSet,
        name: Set(payload.name),
        email: Set(payload.email),
    };

    let inserted_user = new_user
        .insert(&state.db)
        .await
        .map_err(|er| ApiError::internal_bd().add_details(serde_json::json!(er.to_string())))?;

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
        .map_err(|er| ApiError::internal_bd().add_details(serde_json::json!(er.to_string())))?;

    match user {
        Some(user) => Ok((StatusCode::OK, Json(UserId { id: user.id }))),
        None => Err(ApiError::user_not_found()),
    }
}
