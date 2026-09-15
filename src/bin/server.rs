use cofresenhas_rs::server::{build_router, AppState};

const JWT_SECRET_PADRAO: &str = "dev-secret-troque-em-producao";
const ENCRYPTION_SECRET_PADRAO: &str = "dev-encryption-secret-troque-em-producao";

#[tokio::main]
async fn main() {
    let database_path =
        std::env::var("DATABASE_PATH").unwrap_or_else(|_| "cofresenhas.db".to_string());
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| JWT_SECRET_PADRAO.to_string());
    let encryption_secret =
        std::env::var("ENCRYPTION_SECRET").unwrap_or_else(|_| ENCRYPTION_SECRET_PADRAO.to_string());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(5000);

    // `ENCRYPTION_SECRET` é a chave que protege todas as senhas cifradas no
    // banco -- usar o valor padrão (público, está no código-fonte) equivale
    // a não cifrar nada. Avisamos alto em vez de falhar silenciosamente,
    // porque um `.env` ausente é fácil de não notar.
    if jwt_secret == JWT_SECRET_PADRAO || encryption_secret == ENCRYPTION_SECRET_PADRAO {
        eprintln!(
            "\n⚠️  AVISO: JWT_SECRET e/ou ENCRYPTION_SECRET não configurados -- usando valores \
             padrão públicos. Crie um arquivo .env (veja .env.example) antes de guardar senhas \
             de verdade. Trocar o ENCRYPTION_SECRET depois de já ter dados salvos torna-os \
             ilegíveis (a chave de cifra é derivada dele).\n"
        );
    }

    let pool = cofresenhas_rs::server::db::init_pool(&database_path)
        .expect("falha ao inicializar o banco");
    let state = AppState {
        pool,
        jwt_secret,
        encryption_secret,
    };
    let app = build_router(state);

    let addr = format!("0.0.0.0:{port}");
    println!("CofreSenhas API (Rust) ouvindo em http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("falha ao abrir a porta");
    axum::serve(listener, app)
        .await
        .expect("servidor encerrou com erro");
}
