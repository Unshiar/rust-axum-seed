use sea_orm::{EntityTrait, Schema};
pub use sea_orm_migration::prelude::*;

pub mod m20220101_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
    }
}

// 1. Создаем вспомогательный трейт для стирания типов
#[async_trait::async_trait]
pub trait RegisterTable {
    async fn create_table(&self, manager: &SchemaManager) -> Result<(), DbErr>;
}

// 2. Реализуем этот трейт для ЛЮБОЙ сущности SeaORM автоматически
#[async_trait::async_trait]
impl<E> RegisterTable for E
where
    E: EntityTrait + Sync,
{
    async fn create_table(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(manager.get_database_backend());

        // Получаем имя таблицы через Iden
        let table_name = self.to_string();
        println!("🚀 [БД] Check table '{}'...", table_name);

        // Проверяем существование, чтобы не стирать данные при перезапуске
        if !manager.has_table(&table_name).await? {
            println!("🚀 [БД] Таблица '{}' не найдена. Создание...", table_name);
            manager
                .create_table(schema.create_table_from_entity(*self).to_owned())
                .await?;
            println!("✅ [БД] Таблица '{}' успешно создана.", table_name);
        } else {
            println!("🔄 [БД] Таблица '{}' уже существует.", table_name);
        }

        Ok(())
    }
}
