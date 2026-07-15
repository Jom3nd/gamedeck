use tokio::net::TcpListener;
use sea_orm::DatabaseConnection;
use gamedeck::app::create_router;
use gamedeck::config::database::connect_database;
use gamedeck::app_state::AppState;


#[tokio::main]

async fn main() {

    let db:DatabaseConnection = connect_database().await?;
    
    let state = AppState::new(db);

    let app = create_router(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("O Servidor está rodando em http://0.0.0.0:8080");

    axum::serve(listener,app).await?;

}
