use migration::{RegisterTable, SchemaManager};
use sea_orm::{DatabaseConnection, DbErr};

pub mod state;

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
