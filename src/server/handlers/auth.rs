use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use chrono::Utc;
use rand::rngs::OsRng;
use rand::RngCore;
use rusqlite::OptionalExtension;
use serde_json::{json, Value};
use std::sync::OnceLock;

use crate::server::auth::{self, CurrentUser};
use crate::server::error::{AppError, Result};
use crate::server::models::*;
use crate::server::state::AppState;

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>> {
    let conn = state.pool.get()?;

    let existente: Option<i64> = conn
        .query_row(
            "SELECT id FROM usuarios WHERE email = ?1",
            [&req.email],
            |r| r.get(0),
        )
        .optional()?;
    if existente.is_some() {
        return Err(AppError::Conflict("Email já cadastrado.".into()));
    }

    let senha_hash = auth::hash_password(&req.senha)?;
    let mut encryption_salt = [0u8; 16];
    OsRng.fill_bytes(&mut encryption_salt);
    let criado_em = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO usuarios (nome, email, senha_hash, criado_em, encryption_salt) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![req.nome, req.email, senha_hash, criado_em, encryption_salt.to_vec()],
    )?;
    let id = conn.last_insert_rowid();

    let token = auth::generate_token(&state.jwt_secret, id, &req.email, &req.nome, 0)?;
    Ok(Json(AuthResponse {
        token: Some(token),
        two_factor_required: None,
    }))
}

/// Hash falso verificado quando o e-mail não existe, para que "usuário
/// inexistente" e "senha errada" custem o mesmo tempo e deem a mesma resposta.
fn hash_falso() -> &'static str {
    static HASH: OnceLock<String> = OnceLock::new();
    HASH.get_or_init(|| auth::hash_password("senha-falsa-para-tempo-constante").unwrap())
}

fn ip_do_cliente(headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|v| v.trim().to_string())
        .unwrap_or_else(|| "desconhecido".to_string())
}

pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    let chave_email = format!("e:{}", req.email.trim().to_lowercase());
    let chave_ip = format!("i:{}", ip_do_cliente(&headers));
    for chave in [&chave_email, &chave_ip] {
        if let Some(seg) = state.limiter.bloqueado(chave) {
            eprintln!("login bloqueado ({chave}): faltam {seg}s");
            return Err(AppError::TooManyRequests(seg));
        }
    }
    let falhar = |motivo: &str| {
        state.limiter.falhou(&chave_email);
        state.limiter.falhou(&chave_ip);
        eprintln!("login falhou ({chave_ip}, {chave_email}): {motivo}");
        AppError::Unauthorized
    };

    let conn = state.pool.get()?;
    let usuario = conn
        .query_row(
            "SELECT id, nome, email, senha_hash, two_factor_enabled, two_factor_secret, sessao_versao FROM usuarios WHERE email = ?1",
            [&req.email],
            |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                    r.get::<_, bool>(4)?,
                    r.get::<_, Option<String>>(5)?,
                    r.get::<_, i64>(6)?,
                ))
            },
        )
        .optional()?;
    let Some((id, nome, email, senha_hash, two_factor_enabled, two_factor_secret, ver)) = usuario
    else {
        let _ = auth::verify_password(&req.senha, hash_falso());
        return Err(falhar("usuário inexistente"));
    };

    if !auth::verify_password(&req.senha, &senha_hash)? {
        return Err(falhar("senha incorreta"));
    }

    if two_factor_enabled {
        let secret =
            two_factor_secret.ok_or_else(|| AppError::Internal("2FA inconsistente".into()))?;
        match req.totp_code {
            None => {
                return Ok(Json(AuthResponse {
                    token: None,
                    two_factor_required: Some(true),
                }))
            }
            Some(code) => {
                // Aceita o código TOTP ou um código de recuperação (uso único).
                let ok = auth::verify_totp(&secret, &code)
                    || {
                        let hash = auth::hash_codigo_recuperacao(&code);
                        conn.execute(
                        "DELETE FROM codigos_recuperacao WHERE usuario_id = ?1 AND codigo_hash = ?2",
                        rusqlite::params![id, hash],
                    )? == 1
                    };
                if !ok {
                    return Err(falhar("código 2FA inválido"));
                }
            }
        }
    }

    state.limiter.sucesso(&chave_email);
    let token = auth::generate_token(&state.jwt_secret, id, &email, &nome, ver)?;
    Ok(Json(AuthResponse {
        token: Some(token),
        two_factor_required: None,
    }))
}

/// Encerra todas as sessões do usuário (invalida os tokens já emitidos).
pub async fn logout(State(state): State<AppState>, user: CurrentUser) -> Result<Json<Value>> {
    state.pool.get()?.execute(
        "UPDATE usuarios SET sessao_versao = sessao_versao + 1 WHERE id = ?1",
        [user.id],
    )?;
    Ok(Json(json!({ "message": "Sessão encerrada." })))
}

pub async fn setup_2fa(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<Setup2faResponse>> {
    let setup = auth::setup_2fa(&user.email)?;

    let conn = state.pool.get()?;
    conn.execute(
        "UPDATE usuarios SET two_factor_secret = ?1 WHERE id = ?2",
        rusqlite::params![setup.secret_base32, user.id],
    )?;

    // Códigos de recuperação novos substituem os anteriores; só o hash vai
    // para o banco, o texto é devolvido uma única vez.
    conn.execute(
        "DELETE FROM codigos_recuperacao WHERE usuario_id = ?1",
        [user.id],
    )?;
    let codigos: Vec<String> = (0..auth::QTD_CODIGOS_RECUPERACAO)
        .map(|_| auth::gerar_codigo_recuperacao())
        .collect();
    for codigo in &codigos {
        conn.execute(
            "INSERT INTO codigos_recuperacao (usuario_id, codigo_hash) VALUES (?1, ?2)",
            rusqlite::params![user.id, auth::hash_codigo_recuperacao(codigo)],
        )?;
    }

    Ok(Json(Setup2faResponse {
        secret: setup.secret_base32,
        qr_code_uri: setup.otpauth_uri,
        codigos_recuperacao: codigos,
    }))
}

pub async fn verify_2fa(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<Verify2faRequest>,
) -> Result<Json<Value>> {
    let conn = state.pool.get()?;
    let secret: Option<String> = conn
        .query_row(
            "SELECT two_factor_secret FROM usuarios WHERE id = ?1",
            [user.id],
            |r| r.get(0),
        )
        .optional()?
        .flatten();
    let secret = secret.ok_or_else(|| AppError::BadRequest("2FA não configurado.".into()))?;

    if !auth::verify_totp(&secret, &req.code) {
        return Err(AppError::BadRequest("Código inválido.".into()));
    }

    conn.execute(
        "UPDATE usuarios SET two_factor_enabled = 1 WHERE id = ?1",
        [user.id],
    )?;
    Ok(Json(json!({ "message": "2FA ativado com sucesso!" })))
}

pub async fn disable_2fa(State(state): State<AppState>, user: CurrentUser) -> Result<Json<Value>> {
    let conn = state.pool.get()?;
    conn.execute(
        "UPDATE usuarios SET two_factor_enabled = 0, two_factor_secret = NULL WHERE id = ?1",
        [user.id],
    )?;
    conn.execute(
        "DELETE FROM codigos_recuperacao WHERE usuario_id = ?1",
        [user.id],
    )?;
    Ok(Json(json!({ "message": "2FA desativado." })))
}

pub async fn profile(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<ProfileResponse>> {
    let conn = state.pool.get()?;
    let (nome, email, criado_em, two_factor_enabled): (String, String, String, bool) = conn
        .query_row(
            "SELECT nome, email, criado_em, two_factor_enabled FROM usuarios WHERE id = ?1",
            [user.id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )?;

    Ok(Json(ProfileResponse {
        id: user.id,
        nome,
        email,
        criado_em,
        two_factor_enabled,
    }))
}

pub async fn change_password(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<Json<Value>> {
    let conn = state.pool.get()?;
    let senha_hash: String = conn.query_row(
        "SELECT senha_hash FROM usuarios WHERE id = ?1",
        [user.id],
        |r| r.get(0),
    )?;

    if !auth::verify_password(&req.senha_atual, &senha_hash)? {
        return Err(AppError::BadRequest("Senha atual incorreta.".into()));
    }

    let nova_hash = auth::hash_password(&req.nova_senha)?;
    conn.execute(
        "UPDATE usuarios SET senha_hash = ?1 WHERE id = ?2",
        rusqlite::params![nova_hash, user.id],
    )?;
    Ok(Json(json!({ "message": "Senha alterada com sucesso!" })))
}
