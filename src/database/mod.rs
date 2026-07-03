use entities::get_all_tables;
use entities::sea_orm::{DatabaseConnection, DbErr};
use entities::sea_orm_migration::SchemaManager;

pub mod state;

pub async fn register_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let schema_manager = SchemaManager::new(db);

    tracing::debug!("Creating/updating database tables");
    for table in get_all_tables() {
        table.create_table_if_not_exist(&schema_manager).await?;
    }

    Ok(())
}
