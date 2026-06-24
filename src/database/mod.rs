use migration::{SchemaManager, async_trait};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Schema};

pub mod state;

// 1. Создаем вспомогательный трейт для стирания типов
#[async_trait::async_trait]
trait RegisterTable {
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

pub async fn register_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let schema_manager = SchemaManager::new(db);

    let tables: Vec<&dyn RegisterTable> = vec![
        &entities::user::Entity,
        // &crate::entity::post::Entity,   <-- Когда появятся новые, просто раскомментируете
        // &crate::entity::order::Entity,  <-- и добавите их в этот список
    ];

    for table in tables {
        table.create_table(&schema_manager).await?;
    }

    Ok(())
}
