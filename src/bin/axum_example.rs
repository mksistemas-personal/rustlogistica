use logistica::api_routes::app_routes;

#[tokio::main]
async fn main() {
    // constrói nossa aplicação com rotas
    let app = app_routes();

    // executa nossa aplicação
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor Axum rodando em http://localhost:3000");
    println!("POST /api/person disponível");
    axum::serve(listener, app).await.unwrap();
}
