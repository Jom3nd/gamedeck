use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .create_table(
                Table::create()
                    .col(
                        ColumnDef::new(user::Id)
                        .uuif()
                        .not_null()
                        .primary_key(),
                    )
                    .col(
                        ColumnDef::new(user::Username)
                        .string()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(user::Email)
                        .string()
                        .not_null()
                        .unique_key()
                    )
                    .col(
                        ColumnDef::new(user::Password)
                        .string()
                        .not_null()
                    )
                    .to_owned(),  
            )
            .await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
