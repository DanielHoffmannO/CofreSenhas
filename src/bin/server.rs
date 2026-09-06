use cofresenhas_rs::server::{build_router, AppState};

#[tokio::main]
async fn main() {
    let database_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "cofresenhas.db".to_string());
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev-secret-troque-em-producao".to_string());
    let encryption_secret = std::env::var("ENCRYPTION_SECRET")
        .unwrap_or_else(|_| "dev-encryption-secret-troque-em-producao".to_string());
    let port: u16 = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(5000);

    let pool = cofresenhas_rs::server::db::init_pool(&database_path).expect("falha ao inicializar o banco");
    let state = AppState { pool, jwt_secret, encryption_secret };
    let app = build_router(state);

    let addr = format!("0.0.0.0:{port}");
    println!("CofreSenhas API (Rust) ouvindo em http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("falha ao abrir a porta");
    axum::serve(listener, app).await.expect("servidor encerrou com erro");
}
