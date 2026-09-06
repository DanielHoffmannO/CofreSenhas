use axum::extract::State;
use axum::Json;
use chrono::Utc;
use rusqlite::OptionalExtension;

use crate::server::auth::CurrentUser;
use crate::server::error::Result;
use crate::server::models::{AuditLogResponse, AuditRequest};
use crate::server::state::AppState;

pub async fn registrar(State(state): State<AppState>, user: CurrentUser, Json(req): Json<AuditRequest>) -> Result<()> {
    let conn = state.pool.get()?;
    conn.execute(
        "INSERT INTO audit_logs (usuario_id, senha_id, acao, data_hora) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![user.id, req.senha_id, req.acao, Utc::now().to_rfc3339()],
    )?;
    Ok(())
}

pub async fn listar(State(state): State<AppState>, user: CurrentUser) -> Result<Json<Vec<AuditLogResponse>>> {
    let conn = state.pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, senha_id, acao, data_hora FROM audit_logs WHERE usuario_id = ?1 ORDER BY data_hora DESC",
    )?;
    let logs: Vec<(i64, i64, String, String)> = stmt
        .query_map([user.id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
        .collect::<rusqlite::Result<_>>()?;

    let mut result = Vec::with_capacity(logs.len());
    for (id, senha_id, acao, data_hora) in logs {
        let titulo: Option<String> = conn
            .query_row("SELECT titulo FROM senhas WHERE id = ?1", [senha_id], |r| r.get(0))
            .optional()?;
        result.push(AuditLogResponse { id, acao, data_hora, titulo: titulo.unwrap_or_else(|| "(deletada)".to_string()) });
    }

    Ok(Json(result))
}
