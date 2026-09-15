//! Testes de integração do servidor web: sobem o `Router` de verdade (sem
//! abrir socket algum) e disparam requisições HTTP contra ele via
//! `tower::ServiceExt::oneshot` -- é o jeito idiomático de testar um app
//! axum de ponta a ponta, exercitando roteamento + extractors + handlers
//! juntos, sem a lentidão/flakiness de subir um servidor de verdade.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use cofresenhas_rs::server::{build_router, db, AppState};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

fn test_app() -> Router {
    let db_path = tempfile::NamedTempFile::new().unwrap().into_temp_path();
    let db_path_str = db_path.to_str().unwrap().to_string();
    // `into_temp_path` já desvincula o arquivo do guard de limpeza -- o
    // arquivo sobrevive até o processo de teste terminar, o que é o
    // suficiente aqui (cada teste usa seu próprio arquivo isolado).
    std::mem::forget(db_path);

    let pool = db::init_pool(&db_path_str).expect("falha ao inicializar o banco de teste");
    let state = AppState {
        pool,
        jwt_secret: "teste-secret".to_string(),
        encryption_secret: "teste-encryption-secret".to_string(),
    };
    build_router(state)
}

async fn send(
    app: &Router,
    method: &str,
    uri: &str,
    token: Option<&str>,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json");
    if let Some(t) = token {
        builder = builder.header("Authorization", format!("Bearer {t}"));
    }
    let request_body = match body {
        Some(v) => Body::from(v.to_string()),
        None => Body::empty(),
    };
    let request = builder.body(request_body).unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json_body = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap_or(Value::Null)
    };
    (status, json_body)
}

async fn register(app: &Router, email: &str) -> String {
    let (status, body) = send(
        app,
        "POST",
        "/api/auth/register",
        None,
        Some(json!({ "nome": "Teste", "email": email, "senha": "senha123" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    body["token"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn register_and_login_roundtrip() {
    let app = test_app();
    let token = register(&app, "user@teste.com").await;
    assert!(!token.is_empty());

    let (status, body) = send(
        &app,
        "POST",
        "/api/auth/login",
        None,
        Some(json!({ "email": "user@teste.com", "senha": "senha123" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["token"].as_str().is_some());
}

#[tokio::test]
async fn login_with_wrong_password_is_unauthorized() {
    let app = test_app();
    register(&app, "wrongpass@teste.com").await;

    let (status, _) = send(
        &app,
        "POST",
        "/api/auth/login",
        None,
        Some(json!({ "email": "wrongpass@teste.com", "senha": "senha-errada" })),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn protected_route_without_token_is_unauthorized() {
    let app = test_app();
    let (status, _) = send(&app, "GET", "/api/senhas", None, None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn senha_crud_roundtrip() {
    let app = test_app();
    let token = register(&app, "crud@teste.com").await;

    let (status, created) = send(
        &app,
        "POST",
        "/api/senhas",
        Some(&token),
        Some(json!({ "titulo": "GitHub", "login": "daniel", "senha": "segredo123" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(created["senha"], "segredo123");
    let id = created["id"].as_i64().unwrap();

    let (status, listed) = send(&app, "GET", "/api/senhas", Some(&token), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(listed.as_array().unwrap().len(), 1);
    assert_eq!(listed[0]["senha"], "segredo123");

    let (status, updated) = send(
        &app,
        "PUT",
        &format!("/api/senhas/{id}"),
        Some(&token),
        Some(json!({ "titulo": "GitHub", "login": "daniel", "senha": "nova-senha" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(updated["senha"], "nova-senha");

    let (status, _) = send(
        &app,
        "DELETE",
        &format!("/api/senhas/{id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    // deletar de novo confirma que já não existe mais
    let (status, _) = send(
        &app,
        "DELETE",
        &format!("/api/senhas/{id}"),
        Some(&token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn senha_de_um_usuario_nao_aparece_para_outro() {
    let app = test_app();
    let token_a = register(&app, "a@teste.com").await;
    let token_b = register(&app, "b@teste.com").await;

    let (_, created) = send(
        &app,
        "POST",
        "/api/senhas",
        Some(&token_a),
        Some(json!({ "titulo": "Privado", "login": "a", "senha": "x" })),
    )
    .await;
    let id = created["id"].as_i64().unwrap();

    // usuário B não consegue nem enxergar na própria lista...
    let (_, listed_b) = send(&app, "GET", "/api/senhas", Some(&token_b), None).await;
    assert!(listed_b.as_array().unwrap().is_empty());

    // ...nem alterar/apagar a senha de outro usuário pelo id.
    let (status, _) = send(
        &app,
        "DELETE",
        &format!("/api/senhas/{id}"),
        Some(&token_b),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn import_json_skips_duplicates_on_second_run() {
    let app = test_app();
    let token = register(&app, "import@teste.com").await;

    let payload = json!([
        { "titulo": "A", "login": "a", "senha": "1" },
        { "titulo": "B", "login": "b", "senha": "2" },
    ]);

    let (status, first) = send(
        &app,
        "POST",
        "/api/senhas/import/json",
        Some(&token),
        Some(payload.clone()),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(first["imported"], 2);

    let (status, second) = send(
        &app,
        "POST",
        "/api/senhas/import/json",
        Some(&token),
        Some(payload),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(second["imported"], 0);
    assert_eq!(second["skipped"], 2);
}

#[tokio::test]
async fn gerador_respeita_tamanho_pedido() {
    let app = test_app();
    let token = register(&app, "gerador@teste.com").await;

    let (status, body) = send(
        &app,
        "POST",
        "/api/gerador",
        Some(&token),
        Some(
            json!({ "tipo": "aleatorio", "tamanho": 20, "usarMaiusculas": true, "usarNumeros": true, "usarEspeciais": false }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["senha"].as_str().unwrap().chars().count(), 20);
    assert!(!body["senha"]
        .as_str()
        .unwrap()
        .chars()
        .any(|c| !c.is_alphanumeric()));
}

#[tokio::test]
async fn gerador_por_palavras_gera_frase_com_hifen() {
    let app = test_app();
    let token = register(&app, "gerador-palavras@teste.com").await;

    let (status, body) = send(
        &app,
        "POST",
        "/api/gerador",
        Some(&token),
        Some(json!({ "tipo": "palavras", "quantidade": 4, "capitalizar": true, "incluirNumero": true })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let senha = body["senha"].as_str().unwrap();
    let partes: Vec<&str> = senha.split('-').collect();
    // 4 palavras + 1 número no final = 5 pedaços separados por hífen
    assert_eq!(partes.len(), 5);
    assert!(partes[4].chars().all(|c| c.is_ascii_digit()));
}
