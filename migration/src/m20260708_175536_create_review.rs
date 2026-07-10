use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Review::Table).if_not_exists()
                    .col(
                        ColumnDef::new(Review::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Review::UserId)
                        .uuid()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Review::GameId)
                        .uuid()
                        .not_null(),
                    )
                    .col(
                        ColumnDef::new(Review::Score)
                        .small_integer()
                        .not_null()
                    )
                    .col(
                        ColumnDef::new(Review::Title)
                        .string(),
                    )
                    .col(
                        ColumnDef::new(Review::Comment)
                        .text(),
                    )
                    .col(
                        ColumnDef::new(Review::CreatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Review::UpdatedAt)
                        .timestamp_with_time_zone()
                        .not_null()
                        .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk_review_user")
                        .from(Review::Table, Review::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)

                    )
                    .foreign_key(
                        ForeignKey::create()
                        .name("fk_review_game")
                        .from(Review::Table, Review::GameId)
                        .to(Game::Table, Game::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_review_user_game")
                    .table(Review::Table)
                    .col(Review::UserId)
                    .col(Review::GameId)
                    .unique()
                    .to_owned(),
                )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_review_game")
                    .table(Review::Table)
                    .col(Review::GameId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_review_user")
                    .table(Review::Table)
                    .col(Review::UserId)
                    .to_owned(),
                
            )
            .await?;

        Ok(())
    }

    
        

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_index(
                Index::drop()
                    .name("idx_review_user")
                    .table(Review::Table)
                    .to_owned(),
            )   
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_review_game")
                    .table(Review::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_review_user_game")
                    .table(Review::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Review::Table).to_owned())
            .await
        }
}

#[derive(DeriveIden)]
enum Review {
    Table,
    Id,
    UserId,
    GameId,
    Score,
    Title,
    Comment,
    CreatedAt,
    UpdatedAt,
}
