use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create().table(GamePlatform::Table).if_not_exists()
                    .col(
                        ColumnDef::new(GamePlatform::Id)
                        .uuid()
                        .primary_key()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(GamePlatform::Name)
                        .string()
                        .unique_key()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(GamePlatform::Description)
                        .text(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(GamePlatform::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum GamePlatform {
    Table,
    Id,
    Name,
    Description
}
