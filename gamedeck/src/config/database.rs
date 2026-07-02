use SQLx::PGpool;

fn DBconnection() -> SQLx::PGpool {
    let pool = SQLx::PGpool::connect(&std::env::var("DATABASE_URL")).expect("Erro ao conectar no banco de dados");

    return pool;
}