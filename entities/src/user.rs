use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Serialize, ToSchema)]
pub struct UserResponseDto {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "john_doe")]
    pub name: String,
    #[schema(example = "user@example.com")]
    pub email: String,
}

impl From<Model> for UserResponseDto {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            email: model.email,
        }
    }
}
