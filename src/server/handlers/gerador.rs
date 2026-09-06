use axum::extract::State;
use axum::Json;
use chrono::Utc;
use rand::Rng;

use crate::server::auth::CurrentUser;
use crate::server::error::Result;
use crate::server::models::{ForcaSenha, GerarSenhaRequest, GerarSenhaResponse};
use crate::server::state::AppState;

pub async fn gerar(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(req): Json<GerarSenhaRequest>,
) -> Result<Json<GerarSenhaResponse>> {
    let mut chars: Vec<char> = ('a'..='z').collect();
    if req.usar_maiusculas {
        chars.extend('A'..='Z');
    }
    if req.usar_numeros {
        chars.extend('0'..='9');
    }
    if req.usar_especiais {
        chars.extend("!@#$%^&*()_+-=[]{}|;:,.<>?".chars());
    }

    let mut rng = rand::thread_rng();
    let senha: String = (0..req.tamanho).map(|_| chars[rng.gen_range(0..chars.len())]).collect();
    let forca = calcular_forca(&senha);

    let prompt = format!(
        "Tamanho={}, Maiúsculas={}, Números={}, Especiais={}",
        req.tamanho, req.usar_maiusculas, req.usar_numeros, req.usar_especiais
    );

    let conn = state.pool.get()?;
    conn.execute(
        "INSERT INTO historico_geracao (usuario_id, prompt, senha_gerada, forca_senha, criado_em) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![user.id, prompt, senha, format!("{forca:?}"), Utc::now().to_rfc3339()],
    )?;

    Ok(Json(GerarSenhaResponse { senha, forca }))
}

/// Mesma heurística de pontuação do `GeradorSenhaService.CalcularForca`
/// original: um ponto por critério atendido, mapeado para uma categoria.
fn calcular_forca(senha: &str) -> ForcaSenha {
    let mut score = 0;
    if senha.len() >= 8 {
        score += 1;
    }
    if senha.len() >= 12 {
        score += 1;
    }
    if senha.chars().any(|c| c.is_uppercase()) {
        score += 1;
    }
    if senha.chars().any(|c| c.is_ascii_digit()) {
        score += 1;
    }
    if senha.chars().any(|c| !c.is_alphanumeric()) {
        score += 1;
    }

    match score {
        0..=1 => ForcaSenha::Fraca,
        2 => ForcaSenha::Media,
        3 => ForcaSenha::Forte,
        _ => ForcaSenha::MuitoForte,
    }
}
