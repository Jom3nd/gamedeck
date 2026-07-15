pub use sea_orm_migration::prelude::*;

mod migrator;
pub use migrator::Migrator;
mod m20260706_193352_create_users;
mod m20260706_195321_create_games;
mod m20260708_165236_create_genres;
mod m20260708_171133_create_game_plataforms;
mod m20260708_175536_create_review;
mod m20260710_014428_create_game_genres;
mod m20260710_020000_create_library;
mod m20260710_020100_create_refresh_tokens;
