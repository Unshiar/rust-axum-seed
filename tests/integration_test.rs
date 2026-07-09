use axum_app::database::register_tables;
use entities::get_all_tables;
use entities::sea_orm::{Database, DatabaseConnection};
use entities::sea_orm_migration::MigratorTrait;
use sea_orm_migration::SchemaManager;

#[tokio::test]
async fn test_register_tables_with_in_memory_db() {
    let db: DatabaseConnection = Database::connect("sqlite::memory:").await.unwrap();
    let schema_manager = SchemaManager::new(&db);
    let result = register_tables(&db).await;
    assert!(result.is_ok());
    for table in get_all_tables() {
        assert!(table.is_table_exist(&schema_manager).await.unwrap())
    }
}

#[tokio::test]
async fn test_migrator_up_in_memory_db() {
    let db: DatabaseConnection = Database::connect("sqlite::memory:").await.unwrap();
    let schema_manager = SchemaManager::new(&db);
    let result = migration::Migrator::up(&db, None).await;
    assert!(result.is_ok());
    for table in get_all_tables() {
        assert!(table.is_table_exist(&schema_manager).await.unwrap())
    }
}
#[tokio::test]
async fn test_migrator_up_after_register_tables() {
    let db: DatabaseConnection = Database::connect("sqlite::memory:").await.unwrap();
    register_tables(&db).await.unwrap();
    let result = migration::Migrator::up(&db, None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_register_tables_after_migrator_up() {
    let db: DatabaseConnection = Database::connect("sqlite::memory:").await.unwrap();
    migration::Migrator::up(&db, None).await.unwrap();
    let result = register_tables(&db).await;
    assert!(result.is_ok());
}
