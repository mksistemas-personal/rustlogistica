use logistica::api_routes::app_routes;

#[tokio::main]
async fn main() {

    // constrói nossa aplicação com rotas
    let app = app_routes();

    // executa nossa aplicação
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor Axum rodando em http://localhost:3000");
    axum::serve(listener, app).await.unwrap();

/*    println!("Este projeto inclui os frameworks Axum e Diesel!");
    println!("Para rodar o exemplo com Axum, use: cargo run --bin axum_example");
    println!("Para rodar o exemplo com Diesel, use: cargo run --bin diesel_example");
    println!("Para rodar o exemplo de Documentos, use: cargo run --bin document_example");
    println!("Para rodar o exemplo de Serde (JSON), use: cargo run --bin serde_example");
*/
}
