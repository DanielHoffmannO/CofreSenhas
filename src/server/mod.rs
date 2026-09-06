pub mod auth;
pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod state;

use axum::routing::{get, post, put};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};

pub use state::AppState;

/// Monta as rotas da API -- mesmos caminhos e verbos HTTP dos
/// `[Route]`/`[Http*]` do backend ASP.NET original, então o frontend React
/// não precisa mudar uma linha de `api.get(...)`/`api.post(...)`.
pub fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    let auth_routes = Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login))
        .route("/2fa/setup", post(handlers::auth::setup_2fa))
        .route("/2fa/verify", post(handlers::auth::verify_2fa))
        .route("/2fa/disable", post(handlers::auth::disable_2fa))
        .route("/master-password/setup", post(handlers::auth::setup_master_password))
        .route("/master-password/verify", post(handlers::auth::verify_master_password))
        .route("/master-password/status", get(handlers::auth::master_password_status))
        .route("/profile", get(handlers::auth::profile))
        .route("/change-password", put(handlers::auth::change_password));

    let senhas_routes = Router::new()
        .route("/", get(handlers::senhas::list).post(handlers::senhas::create))
        .route("/export/json", get(handlers::senhas::export_json))
        .route("/export/csv", get(handlers::senhas::export_csv))
        .route("/import/json", post(handlers::senhas::import_json))
        .route(
            "/:id",
            get(handlers::senhas::get_by_id).put(handlers::senhas::update).delete(handlers::senhas::delete),
        )
        .route("/:id/historico", get(handlers::senhas::historico))
        .route("/:id/restaurar/:versao_id", post(handlers::senhas::restaurar_versao));

    let gerador_routes = Router::new().route("/", post(handlers::gerador::gerar));

    let audit_routes =
        Router::new().route("/", post(handlers::audit::registrar).get(handlers::audit::listar));

    Router::new()
        .nest("/api/auth", auth_routes)
        .nest("/api/senhas", senhas_routes)
        .nest("/api/gerador", gerador_routes)
        .nest("/api/audit", audit_routes)
        .route("/health", get(|| async { "ok" }))
        .layer(cors)
        .with_state(state)
}
