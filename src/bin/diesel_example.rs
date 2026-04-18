use diesel::Connection;
use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL deve estar configurada no arquivo .env");
    
    println!("Diesel configurado com Postgres.");
    println!("DATABASE_URL: {}", database_url);
    
    let _connection = diesel::pg::PgConnection::establish(&database_url);
    println!("Tentativa de conexão concluída (Pode falhar se o banco não estiver rodando).");
}
