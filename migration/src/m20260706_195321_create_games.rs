use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create().table(Game::Table).if_not_exists()

                    .col(
                        ColumnDef::new(Game::Id)
                        .uuid()
                        .not_null()
                        .primary_key()
                    )
                    .col(
                        ColumnDef::new(Game::Title)
                        .string()
                        .not_null(),

                    )
                    .col(
                        ColumnDef::new(Game::Description)
                        .text(),
                    )
                    .col(
                        ColumnDef::new(Game::Developer)
                        .string()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Game::Price)
                        .decimal_len(10,2)
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Game::GenreId)
                        .uuid()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Game::ReleaseDate)
                        .date()
                        .null(),
                    )
                    .col(
                        ColumnDef::new(Game::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Game::UpdatedAt)
                        .timestamp_with_time_zone()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Game::GameUrl)
                        .null()
                        .string(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk_game_genre")
                        .from(Game::Table, Game::GenreId)
                        .to(Genre::Table , Genre::Id)
                        .on_delete(ForeignKeyAction::Restrict)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
    Title,
    Description,
    Developer,
    Price,
    GenreId,
    ReleaseDate,
    CreatedAt,
    UpdatedAt,
    GameUrl
}
