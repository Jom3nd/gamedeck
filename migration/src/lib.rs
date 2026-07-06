pub use sea_orm_migration::prelude::*;

mod migrator;
pub use migrator::Migrator;
mod m20260706_193352_create_users;
