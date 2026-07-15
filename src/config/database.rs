use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn connect_database() -> Result<DatabaseConnection, DbErr> {
    let database_url = std::env::var("DATABASE_URL")
    .expect("O DATABASE_URL não foi encontrado");

    let mut options = ConnectOptions::new(database_url);


    options
        .sqlx_logging(true)
        .max_connections(10)
        .min_connections(2);

    Database::connect(options).await
}