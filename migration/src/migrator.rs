pub use sea_orm_migration::prelude::*;

mod m20260706_000001_create_users;
mod m20260706_000002_create_games;
mod m20260706_000003_create_library;
mod m20260706_000004_create_reviews;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260706_000001_create_users::Migration),
            Box::new(m20260706_000002_create_games::Migration),
            Box::new(m20260706_000003_create_library::Migration),
            Box::new(m20260706_000004_create_reviews::Migration),
        ]
    }
}
