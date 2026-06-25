use entities::get_all_tables;
use entities::sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for table in get_all_tables() {
            table.create_table_force(manager).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for table in get_all_tables().iter().rev() {
            table.drop_table_if_exists(manager).await?;
        }

        Ok(())
    }
}
