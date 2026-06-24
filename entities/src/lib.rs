pub mod user;

use sea_orm::{DbErr, EntityTrait, Schema};
use sea_orm_migration::prelude::Table;
use sea_orm_migration::{SchemaManager, async_trait};

// 1. Единый трейт для управления схемами, доступный везде
#[async_trait::async_trait]
pub trait ManageSchema {
    async fn create_table_if_not_exist(&self, manager: &SchemaManager) -> Result<(), DbErr>;
    async fn create_table_force(&self, manager: &SchemaManager) -> Result<(), DbErr>;
    async fn drop_table_if_exists(&self, manager: &SchemaManager) -> Result<(), DbErr>;
}

// 2. Универсальная автоматическая реализация для всех моделей
#[async_trait::async_trait]
impl<E> ManageSchema for E
where
    E: EntityTrait + Sync,
{
    // Метод для локального Dev (с проверкой, есть ли таблица)
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

    // Метод для первой миграции (жесткое создание без лишних проверок)
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

// 3. Функция возвращает список всех сущностей проекта
pub fn get_all_tables() -> Vec<&'static (dyn ManageSchema + Sync)> {
    vec![
        &user::Entity,
        // &crate::post::Entity, <-- При добавлении новой модели пишем ТОЛЬКО сюда!
    ]
}
