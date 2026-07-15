use sea_orm_migration::{prelude::*, schema::*};
use crate::schema::{Game, User};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Library::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Library::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Library::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Library::GameId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Library::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_library_user")
                            .from(Library::Table, Library::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_library_game")
                            .from(Library::Table, Library::GameId)
                            .to(Game::Table, Game::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_library_user_game")
                    .table(Library::Table)
                    .col(Library::UserId)
                    .col(Library::GameId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_library_user_game")
                    .table(Library::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Library::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Library {
    Table,
    Id,
    UserId,
    GameId,
    CreatedAt,
}
