use entities::user::Entity as User;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Schema;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. ПРОВЕРКА ДЛЯ PRODUCTION
        let schema = Schema::new(manager.get_database_backend());
        if !manager.has_table(User.to_string()).await? {
            return manager
                .create_table(schema.create_table_from_entity(User).to_owned())
                .await;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(User).to_owned())
            .await?;

        Ok(())
    }
}
