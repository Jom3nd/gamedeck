use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum Game {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum Genre {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum Platform {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum Review {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum Library {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum RefreshToken {
    Table,
    Id,
}

#[derive(DeriveIden)]
pub enum GameGenre {
    Table,
    GameId,
    GenreId,
}

#[derive(DeriveIden)]
pub enum GamePlatform {
    Table,
    GameId,
    PlatformId,
}