use sea_orm::{EntityTrait, Schema};
use sea_orm_migration::prelude::*;

// 1. Повторяем наш трейт-обертку для стирания типов внутри миграции
#[async_trait::async_trait]
trait MigrationTable {
    async fn create_in_migration(&self, manager: &SchemaManager) -> Result<(), DbErr>;
}

#[async_trait::async_trait]
impl<E> MigrationTable for E
where
    E: EntityTrait + Sync,
{
    async fn create_in_migration(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(manager.get_database_backend());
        // Создаем таблицу БЕЗ проверки has_table, так как первая миграция обязана выполняться на чистой БД
        manager
            .create_table(schema.create_table_from_entity(*self).to_owned())
            .await
    }
}
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 2. Собираем все текущие модели в один список
        let tables: Vec<&(dyn MigrationTable + Sync)> = vec![
            &entities::user::Entity,
            // &entity::post::Entity, <-- Новые таблицы на старте проекта добавляются сюда
        ];

        // 3. Создаем их по очереди
        for table in tables {
            table.create_in_migration(manager).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(entities::user::Entity)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
