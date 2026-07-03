pub mod user;
pub use sea_orm;
pub use sea_orm_migration;

use sea_orm::{DbErr, EntityTrait, Schema};
use sea_orm_migration::prelude::Table;
use sea_orm_migration::{SchemaManager, async_trait};

#[async_trait::async_trait]
pub trait ManageSchema {
    async fn create_table_if_not_exist(&self, manager: &SchemaManager) -> Result<(), DbErr>;
    async fn create_table_force(&self, manager: &SchemaManager) -> Result<(), DbErr>;
    async fn drop_table_if_exists(&self, manager: &SchemaManager) -> Result<(), DbErr>;
}

#[async_trait::async_trait]
impl<E> ManageSchema for E
where
    E: EntityTrait + Sync,
{
    async fn create_table_if_not_exist(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table_name = self.to_string();
        if !manager.has_table(&table_name).await? {
            println!("🚀 [БД] Таблица '{}' не найдена. Создание...", table_name);
            self.create_table_force(manager).await?;
        } else {
            println!("🔄 [БД] Таблица '{}' уже существует.", table_name);
        }
        Ok(())
    }

    async fn create_table_force(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(manager.get_database_backend());
        manager
            .create_table(schema.create_table_from_entity(*self).to_owned())
            .await
    }

    async fn drop_table_if_exists(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(*self).if_exists().to_owned())
            .await
    }
}

pub fn get_all_tables() -> Vec<&'static (dyn ManageSchema + Sync)> {
    vec![&user::Entity]
}
