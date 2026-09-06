use axum::extract::{Path, Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use serde::Deserialize;
use serde_json::json;

use crate::crypto::VaultCipher;
use crate::server::auth::CurrentUser;
use crate::server::error::{AppError, Result};
use crate::server::models::*;
use crate::server::state::AppState;

/// Deriva a chave de cifra do usuário e monta o `VaultCipher` -- reaproveita
/// o mesmo `crypto` da CLI, só troca de onde vem o salt (aqui, uma coluna
/// da tabela `usuarios`; lá, o `MasterRecord` do cofre local).
fn cipher_for_user(conn: &Connection, state: &AppState, user_id: i64) -> Result<VaultCipher> {
    let salt: Vec<u8> = conn.query_row("SELECT encryption_salt FROM usuarios WHERE id = ?1", [user_id], |r| r.get(0))?;
    let key = crate::crypto::derive_encryption_key(&state.encryption_secret, &salt)?;
    Ok(VaultCipher::new(&key))
}

fn row_to_response(row: SenhaRow, cipher: &VaultCipher) -> Result<SenhaResponse> {
    let senha = cipher.decrypt(&row.senha_ciphertext, &row.senha_nonce)?;
    Ok(SenhaResponse {
        id: row.id,
        titulo: row.titulo,
        login: row.login,
        senha,
        url: row.url,
        notas: row.notas,
        categoria: row.categoria,
        criado_em: row.criado_em,
    })
}

fn fetch_senha_row(conn: &Connection, id: i64, user_id: i64) -> Result<SenhaRow> {
    conn.query_row(
        "SELECT id, titulo, login, senha_ciphertext, senha_nonce, url, notas, categoria, criado_em, atualizado_em
         FROM senhas WHERE id = ?1 AND usuario_id = ?2",
        params![id, user_id],
        |r| {
            Ok(SenhaRow {
                id: r.get(0)?,
                titulo: r.get(1)?,
                login: r.get(2)?,
                senha_ciphertext: r.get(3)?,
                senha_nonce: r.get(4)?,
                url: r.get(5)?,
                notas: r.get(6)?,
                categoria: r.get(7)?,
                criado_em: r.get(8)?,
                atualizado_em: r.get(9)?,
            })
        },
    )
    .optional()?
    .ok_or(AppError::NotFound)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListQuery {
    page: Option<i64>,
    page_size: Option<i64>,
}

pub async fn list(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(q): Query<ListQuery>,
) -> Result<Response> {
    let conn = state.pool.get()?;
    let cipher = cipher_for_user(&conn, &state, user.id)?;

    let mut stmt = conn.prepare(
        "SELECT id, titulo, login, senha_ciphertext, senha_nonce, url, notas, categoria, criado_em, atualizado_em
         FROM senhas WHERE usuario_id = ?1 ORDER BY criado_em DESC",
    )?;
    let rows: Vec<SenhaRow> = stmt
        .query_map([user.id], |r| {
            Ok(SenhaRow {
                id: r.get(0)?,
                titulo: r.get(1)?,
                login: r.get(2)?,
                senha_ciphertext: r.get(3)?,
                senha_nonce: r.get(4)?,
                url: r.get(5)?,
                notas: r.get(6)?,
                categoria: r.get(7)?,
                criado_em: r.get(8)?,
                atualizado_em: r.get(9)?,
            })
        })?
        .collect::<rusqlite::Result<_>>()?;

    match q.page {
        Some(page) if page > 0 => {
            let page_size = q.page_size.unwrap_or(20).clamp(1, 100);
            let total_count = rows.len() as i64;
            let start = ((page - 1) * page_size) as usize;
            let page_items: Vec<SenhaResponse> = rows
                .into_iter()
                .skip(start)
                .take(page_size as usize)
                .map(|row| row_to_response(row, &cipher))
                .collect::<Result<_>>()?;
            Ok(Json(PagedResponse::new(page_items, page, page_size, total_count)).into_response())
        }
        _ => {
            let items: Vec<SenhaResponse> =
                rows.into_iter().map(|row| row_to_response(row, &cipher)).collect::<Result<_>>()?;
            Ok(Json(items).into_response())
        }
    }
}

pub async fn get_by_id(State(state): State<AppState>, user: CurrentUser, Path(id): Path<i64>) -> Result<Json<SenhaResponse>> {
    let conn = state.pool.get()?;
    let cipher = cipher_for_user(&conn, &state, user.id)?;
    let row = fetch_senha_row(&conn, id, user.id)?;
    Ok(Json(row_to_response(row, &cipher)?))
}

pub async fn create(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<CriarSenhaRequest>,
) -> Result<Json<SenhaResponse>> {
    let conn = state.pool.get()?;
    let cipher = cipher_for_user(&conn, &state, user.id)?;
    let (ciphertext, nonce) = cipher.encrypt(&req.senha)?;
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO senhas (usuario_id, titulo, login, senha_ciphertext, senha_nonce, url, notas, categoria, criado_em, atualizado_em)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)",
        params![user.id, req.titulo, req.login, ciphertext, nonce, req.url, req.notas, req.categoria, now],
    )?;
    let id = conn.last_insert_rowid();

    Ok(Json(SenhaResponse {
        id,
        titulo: req.titulo,
        login: req.login,
        senha: req.senha,
        url: req.url,
        notas: req.notas,
        categoria: req.categoria,
        criado_em: now,
    }))
}

pub async fn update(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i64>,
    Json(req): Json<CriarSenhaRequest>,
) -> Result<Json<SenhaResponse>> {
    let conn = state.pool.get()?;
    let existing = fetch_senha_row(&conn, id, user.id)?;

    // Guarda o estado anterior no histórico antes de sobrescrever --
    // mesma lógica do `SenhaService.AtualizarAsync` original.
    conn.execute(
        "INSERT INTO senha_versoes (senha_id, titulo, login, senha_ciphertext, senha_nonce, alterado_em)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![existing.id, existing.titulo, existing.login, existing.senha_ciphertext, existing.senha_nonce, existing.atualizado_em],
    )?;

    let cipher = cipher_for_user(&conn, &state, user.id)?;
    let (ciphertext, nonce) = cipher.encrypt(&req.senha)?;
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "UPDATE senhas SET titulo = ?1, login = ?2, senha_ciphertext = ?3, senha_nonce = ?4, url = ?5, notas = ?6, categoria = ?7, atualizado_em = ?8
         WHERE id = ?9",
        params![req.titulo, req.login, ciphertext, nonce, req.url, req.notas, req.categoria, now, id],
    )?;

    Ok(Json(SenhaResponse {
        id,
        titulo: req.titulo,
        login: req.login,
        senha: req.senha,
        url: req.url,
        notas: req.notas,
        categoria: req.categoria,
        criado_em: existing.criado_em,
    }))
}

pub async fn delete(State(state): State<AppState>, user: CurrentUser, Path(id): Path<i64>) -> Result<Response> {
    let conn = state.pool.get()?;
    let affected = conn.execute("DELETE FROM senhas WHERE id = ?1 AND usuario_id = ?2", params![id, user.id])?;
    if affected == 0 {
        return Err(AppError::NotFound);
    }
    Ok(axum::http::StatusCode::NO_CONTENT.into_response())
}

pub async fn export_json(State(state): State<AppState>, user: CurrentUser) -> Result<Response> {
    let conn = state.pool.get()?;
    let cipher = cipher_for_user(&conn, &state, user.id)?;

    let mut stmt = conn.prepare(
        "SELECT id, titulo, login, senha_ciphertext, senha_nonce, url, notas, categoria, criado_em, atualizado_em
         FROM senhas WHERE usuario_id = ?1 ORDER BY criado_em DESC",
    )?;
    let items: Vec<SenhaResponse> = stmt
        .query_map([user.id], |r| {
            Ok(SenhaRow {
                id: r.get(0)?,
                titulo: r.get(1)?,
                login: r.get(2)?,
                senha_ciphertext: r.get(3)?,
                senha_nonce: r.get(4)?,
                url: r.get(5)?,
                notas: r.get(6)?,
                categoria: r.get(7)?,
                criado_em: r.get(8)?,
                atualizado_em: r.get(9)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?
        .into_iter()
        .map(|row| row_to_response(row, &cipher))
        .collect::<Result<_>>()?;

    let json_body = serde_json::to_string_pretty(&items).map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((
        [
            (header::CONTENT_TYPE, "application/json"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"cofre-senhas-export.json\""),
        ],
        json_body,
    )
        .into_response())
}

pub async fn export_csv(State(state): State<AppState>, user: CurrentUser) -> Result<Response> {
    let conn = state.pool.get()?;
    let cipher = cipher_for_user(&conn, &state, user.id)?;

    let mut stmt = conn.prepare(
        "SELECT id, titulo, login, senha_ciphertext, senha_nonce, url, notas, categoria, criado_em, atualizado_em
         FROM senhas WHERE usuario_id = ?1 ORDER BY criado_em DESC",
    )?;
    let items: Vec<SenhaResponse> = stmt
        .query_map([user.id], |r| {
            Ok(SenhaRow {
                id: r.get(0)?,
                titulo: r.get(1)?,
                login: r.get(2)?,
                senha_ciphertext: r.get(3)?,
                senha_nonce: r.get(4)?,
                url: r.get(5)?,
                notas: r.get(6)?,
                categoria: r.get(7)?,
                criado_em: r.get(8)?,
                atualizado_em: r.get(9)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?
        .into_iter()
        .map(|row| row_to_response(row, &cipher))
        .collect::<Result<_>>()?;

    let mut csv = String::from("Titulo,Login,Senha,URL,Categoria,Notas\n");
    for item in &items {
        csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            csv_escape(&item.titulo),
            csv_escape(&item.login),
            csv_escape(&item.senha),
            csv_escape(item.url.as_deref().unwrap_or("")),
            csv_escape(&item.categoria),
            csv_escape(item.notas.as_deref().unwrap_or("")),
        ));
    }

    Ok((
        [
            (header::CONTENT_TYPE, "text/csv"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"cofre-senhas-export.csv\""),
        ],
        csv,
    )
        .into_response())
}

fn csv_escape(value: &str) -> String {
    if value.is_empty() {
        return String::new();
    }
    if value.contains('"') || value.contains(',') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

pub async fn import_json(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(items): Json<Vec<CriarSenhaRequest>>,
) -> Result<Json<serde_json::Value>> {
    let conn = state.pool.get()?;
    let cipher = cipher_for_user(&conn, &state, user.id)?;
    let mut count = 0;

    for item in items {
        let (ciphertext, nonce) = cipher.encrypt(&item.senha)?;
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO senhas (usuario_id, titulo, login, senha_ciphertext, senha_nonce, url, notas, categoria, criado_em, atualizado_em)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)",
            params![user.id, item.titulo, item.login, ciphertext, nonce, item.url, item.notas, item.categoria, now],
        )?;
        count += 1;
    }

    Ok(Json(json!({ "imported": count })))
}

pub async fn historico(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<i64>,
) -> Result<Json<Vec<SenhaVersaoResponse>>> {
    let conn = state.pool.get()?;
    // Confirma que a senha pertence ao usuário antes de mostrar o histórico.
    fetch_senha_row(&conn, id, user.id)?;
    let cipher = cipher_for_user(&conn, &state, user.id)?;

    let mut stmt = conn.prepare(
        "SELECT id, titulo, login, senha_ciphertext, senha_nonce, alterado_em FROM senha_versoes WHERE senha_id = ?1 ORDER BY alterado_em DESC",
    )?;
    let versoes: Vec<SenhaVersaoResponse> = stmt
        .query_map([id], |r| {
            Ok(SenhaVersaoRow {
                id: r.get(0)?,
                titulo: r.get(1)?,
                login: r.get(2)?,
                senha_ciphertext: r.get(3)?,
                senha_nonce: r.get(4)?,
                alterado_em: r.get(5)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?
        .into_iter()
        .map(|v| {
            let senha = cipher.decrypt(&v.senha_ciphertext, &v.senha_nonce)?;
            Ok(SenhaVersaoResponse { id: v.id, titulo: v.titulo, login: v.login, senha, alterado_em: v.alterado_em })
        })
        .collect::<Result<_>>()?;

    Ok(Json(versoes))
}

pub async fn restaurar_versao(
    State(state): State<AppState>,
    user: CurrentUser,
    Path((senha_id, versao_id)): Path<(i64, i64)>,
) -> Result<Json<SenhaResponse>> {
    let conn = state.pool.get()?;
    let existing = fetch_senha_row(&conn, senha_id, user.id)?;

    let versao: (String, String, Vec<u8>, Vec<u8>) = conn
        .query_row(
            "SELECT titulo, login, senha_ciphertext, senha_nonce FROM senha_versoes WHERE id = ?1 AND senha_id = ?2",
            params![versao_id, senha_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )
        .optional()?
        .ok_or(AppError::NotFound)?;

    // Guarda o estado atual como uma nova versão antes de restaurar.
    conn.execute(
        "INSERT INTO senha_versoes (senha_id, titulo, login, senha_ciphertext, senha_nonce, alterado_em)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![existing.id, existing.titulo, existing.login, existing.senha_ciphertext, existing.senha_nonce, existing.atualizado_em],
    )?;

    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE senhas SET titulo = ?1, login = ?2, senha_ciphertext = ?3, senha_nonce = ?4, atualizado_em = ?5 WHERE id = ?6",
        params![versao.0, versao.1, versao.2, versao.3, now, senha_id],
    )?;

    let cipher = cipher_for_user(&conn, &state, user.id)?;
    let senha = cipher.decrypt(&versao.2, &versao.3)?;

    Ok(Json(SenhaResponse {
        id: senha_id,
        titulo: versao.0,
        login: versao.1,
        senha,
        url: existing.url,
        notas: existing.notas,
        categoria: existing.categoria,
        criado_em: existing.criado_em,
    }))
}
