use sea_orm_migration::sea_orm::Schema;
use sea_orm_migration::{prelude::*, schema::*};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. ПРОВЕРКА ДЛЯ PRODUCTION
        if manager.has_table("users").await? {
            // Если таблица уже есть на живом сервере, мы НИЧЕГО не делаем.
            // Продакшен-сервер уже прошел всю историю изменений ранее.
        }

        let schema = Schema::new(manager.get_database_backend());

        // manager
        //     .create_table(schema.create_table_from_entity(User).to_owned())
        //     .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        Ok(())
    }
}
