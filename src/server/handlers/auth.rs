use axum::extract::State;
use axum::Json;
use chrono::Utc;
use rand::rngs::OsRng;
use rand::RngCore;
use rusqlite::OptionalExtension;
use serde_json::{json, Value};

use crate::server::auth::{self, CurrentUser};
use crate::server::error::{AppError, Result};
use crate::server::models::*;
use crate::server::state::AppState;

pub async fn register(State(state): State<AppState>, Json(req): Json<RegisterRequest>) -> Result<Json<AuthResponse>> {
    let conn = state.pool.get()?;

    let existente: Option<i64> = conn
        .query_row("SELECT id FROM usuarios WHERE email = ?1", [&req.email], |r| r.get(0))
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

    let token = auth::generate_token(&state.jwt_secret, id, &req.email, &req.nome)?;
    Ok(Json(AuthResponse { token: Some(token), two_factor_required: None }))
}

pub async fn login(State(state): State<AppState>, Json(req): Json<LoginRequest>) -> Result<Json<AuthResponse>> {
    let conn = state.pool.get()?;

    let usuario = conn
        .query_row(
            "SELECT id, nome, email, senha_hash, two_factor_enabled, two_factor_secret FROM usuarios WHERE email = ?1",
            [&req.email],
            |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                    r.get::<_, bool>(4)?,
                    r.get::<_, Option<String>>(5)?,
                ))
            },
        )
        .optional()?
        .ok_or(AppError::Unauthorized)?;
    let (id, nome, email, senha_hash, two_factor_enabled, two_factor_secret) = usuario;

    if !auth::verify_password(&req.senha, &senha_hash)? {
        return Err(AppError::Unauthorized);
    }

    if two_factor_enabled {
        let secret = two_factor_secret.ok_or_else(|| AppError::Internal("2FA inconsistente".into()))?;
        match req.totp_code {
            None => return Ok(Json(AuthResponse { token: None, two_factor_required: Some(true) })),
            Some(code) if !auth::verify_totp(&secret, &code) => {
                return Err(AppError::Unauthorized);
            }
            Some(_) => {}
        }
    }

    let token = auth::generate_token(&state.jwt_secret, id, &email, &nome)?;
    Ok(Json(AuthResponse { token: Some(token), two_factor_required: None }))
}

pub async fn setup_2fa(State(state): State<AppState>, user: CurrentUser) -> Result<Json<Setup2faResponse>> {
    let setup = auth::setup_2fa(&user.email)?;

    let conn = state.pool.get()?;
    conn.execute(
        "UPDATE usuarios SET two_factor_secret = ?1 WHERE id = ?2",
        rusqlite::params![setup.secret_base32, user.id],
    )?;

    Ok(Json(Setup2faResponse { secret: setup.secret_base32, qr_code_uri: setup.otpauth_uri }))
}

pub async fn verify_2fa(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<Verify2faRequest>,
) -> Result<Json<Value>> {
    let conn = state.pool.get()?;
    let secret: Option<String> = conn
        .query_row("SELECT two_factor_secret FROM usuarios WHERE id = ?1", [user.id], |r| r.get(0))
        .optional()?
        .flatten();
    let secret = secret.ok_or_else(|| AppError::BadRequest("2FA não configurado.".into()))?;

    if !auth::verify_totp(&secret, &req.code) {
        return Err(AppError::BadRequest("Código inválido.".into()));
    }

    conn.execute("UPDATE usuarios SET two_factor_enabled = 1 WHERE id = ?1", [user.id])?;
    Ok(Json(json!({ "message": "2FA ativado com sucesso!" })))
}

pub async fn disable_2fa(State(state): State<AppState>, user: CurrentUser) -> Result<Json<Value>> {
    let conn = state.pool.get()?;
    conn.execute(
        "UPDATE usuarios SET two_factor_enabled = 0, two_factor_secret = NULL WHERE id = ?1",
        [user.id],
    )?;
    Ok(Json(json!({ "message": "2FA desativado." })))
}

pub async fn setup_master_password(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<SetupMasterPasswordRequest>,
) -> Result<Json<Value>> {
    let hash = auth::hash_password(&req.master_password)?;
    let conn = state.pool.get()?;
    conn.execute("UPDATE usuarios SET master_password_hash = ?1 WHERE id = ?2", rusqlite::params![hash, user.id])?;
    Ok(Json(json!({ "message": "Master password configurada com sucesso!" })))
}

pub async fn verify_master_password(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<VerifyMasterPasswordRequest>,
) -> Result<Json<Value>> {
    let conn = state.pool.get()?;
    let hash: Option<String> = conn
        .query_row("SELECT master_password_hash FROM usuarios WHERE id = ?1", [user.id], |r| r.get(0))
        .optional()?
        .flatten();

    let valid = match hash {
        Some(hash) => auth::verify_password(&req.master_password, &hash)?,
        None => false,
    };

    if valid {
        Ok(Json(json!({ "valid": true })))
    } else {
        Err(AppError::BadRequest("Master password incorreta.".into()))
    }
}

pub async fn master_password_status(
    State(state): State<AppState>,
    user: CurrentUser,
) -> Result<Json<MasterPasswordStatusResponse>> {
    let conn = state.pool.get()?;
    let hash: Option<String> = conn
        .query_row("SELECT master_password_hash FROM usuarios WHERE id = ?1", [user.id], |r| r.get(0))
        .optional()?
        .flatten();
    Ok(Json(MasterPasswordStatusResponse { is_configured: hash.is_some() }))
}

pub async fn profile(State(state): State<AppState>, user: CurrentUser) -> Result<Json<ProfileResponse>> {
    let conn = state.pool.get()?;
    let (nome, email, criado_em, two_factor_enabled, master_password_hash): (String, String, String, bool, Option<String>) =
        conn.query_row(
            "SELECT nome, email, criado_em, two_factor_enabled, master_password_hash FROM usuarios WHERE id = ?1",
            [user.id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
        )?;

    Ok(Json(ProfileResponse {
        id: user.id,
        nome,
        email,
        criado_em,
        two_factor_enabled,
        master_password_configured: master_password_hash.is_some(),
    }))
}

pub async fn change_password(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<Json<Value>> {
    let conn = state.pool.get()?;
    let senha_hash: String = conn.query_row("SELECT senha_hash FROM usuarios WHERE id = ?1", [user.id], |r| r.get(0))?;

    if !auth::verify_password(&req.senha_atual, &senha_hash)? {
        return Err(AppError::BadRequest("Senha atual incorreta.".into()));
    }

    let nova_hash = auth::hash_password(&req.nova_senha)?;
    conn.execute("UPDATE usuarios SET senha_hash = ?1 WHERE id = ?2", rusqlite::params![nova_hash, user.id])?;
    Ok(Json(json!({ "message": "Senha alterada com sucesso!" })))
}
