pub mod auth;
pub mod db;
pub mod dominio;
pub mod error;
pub mod handlers;
pub mod limiter;
pub mod models;
pub mod state;

use axum::http::{header, HeaderName, HeaderValue, Method};
use axum::routing::{get, post, put};
use axum::Router;
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;

pub use state::AppState;

/// Monta as rotas da API -- mesmos caminhos e verbos HTTP dos
/// `[Route]`/`[Http*]` do backend ASP.NET original, então o frontend React
/// não precisa mudar uma linha de `api.get(...)`/`api.post(...)`.
pub fn build_router(state: AppState) -> Router {
    // O frontend fala com a API na mesma origem (proxy), então CORS só
    // precisa liberar origens explícitas: CORS_ORIGINS (lista separada por
    // vírgula) ou, por padrão, o endereço do cofre na LAN.
    let origens: Vec<HeaderValue> = std::env::var("CORS_ORIGINS")
        .unwrap_or_else(|_| "https://cofre.hoffserv.lan".to_string())
        .split(',')
        .filter_map(|o| HeaderValue::from_str(o.trim()).ok())
        .collect();
    let cors = CorsLayer::new()
        .allow_origin(origens)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

    let auth_routes = Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login))
        .route("/2fa/setup", post(handlers::auth::setup_2fa))
        .route("/2fa/verify", post(handlers::auth::verify_2fa))
        .route("/2fa/disable", post(handlers::auth::disable_2fa))
        .route("/logout", post(handlers::auth::logout))
        .route("/profile", get(handlers::auth::profile))
        .route("/change-password", put(handlers::auth::change_password));

    let senhas_routes = Router::new()
        .route(
            "/",
            get(handlers::senhas::list).post(handlers::senhas::create),
        )
        .route("/export/json", get(handlers::senhas::export_json))
        .route("/export/csv", get(handlers::senhas::export_csv))
        .route("/import/json", post(handlers::senhas::import_json))
        .route(
            "/:id",
            put(handlers::senhas::update).delete(handlers::senhas::delete),
        );

    let gerador_routes = Router::new().route("/", post(handlers::gerador::gerar));

    Router::new()
        .nest("/api/auth", auth_routes)
        .nest("/api/senhas", senhas_routes)
        .nest("/api/gerador", gerador_routes)
        .route("/health", get(|| async { "ok" }))
        .layer(cors)
        .layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .with_state(state)
}
