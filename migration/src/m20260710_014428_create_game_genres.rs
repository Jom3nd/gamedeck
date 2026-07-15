use sea_orm_migration::{prelude::*, schema::*};
use crate::schema::{Game,Genre,GameGenre};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(GameGenre::Table).if_not_exists()
                    .col(
                        ColumnDef::new(GameGenre::GameId)
                        .integer()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(GameGenre::GenreId)
                        .integer()
                        .not_null(),
                    )
                    .primary_key(
                        Index::create()
                        .col(GameGenre::GameId)
                        .col(GameGenre::GenreId)
                        .to_owned(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk_game_genre_game")
                        .from(GameGenre::Table, GameGenre::GameId)
                        .to(Game::Table, Game::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk_game_genre_genre")
                        .from(GameGenre::Table, GameGenre::GenreId)
                        .to(Genre::Table, Genre::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(GameGenre::Table).to_owned())
            .await
    }
}