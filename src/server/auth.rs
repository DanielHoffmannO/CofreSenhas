use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use totp_rs::{Algorithm, Secret, TOTP};

use super::error::{AppError, Result};
use super::state::AppState;

/// Claims embutidos no JWT -- equivalente aos `Claim`s do
/// `ClaimTypes.NameIdentifier`/`Email`/`Name` no `AuthService.GerarToken`
/// original. `sub` (subject) é a convenção padrão de JWT para "de quem é
/// este token"; aqui guardamos o id do usuário como string.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub nome: String,
    pub exp: usize,
    /// Versão da sessão do usuário; o logout a incrementa e invalida o token.
    #[serde(default)]
    pub ver: i64,
}

pub fn generate_token(
    secret: &str,
    user_id: i64,
    email: &str,
    nome: &str,
    ver: i64,
) -> Result<String> {
    let exp = now_secs() + 8 * 3600; // expira em 8h, igual ao original
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        nome: nome.to_string(),
        exp: exp as usize,
        ver,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("falha ao gerar token: {e}")))
}

fn decode_token(secret: &str, token: &str) -> Result<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized)
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// O usuário autenticado da requisição atual, extraído do header
/// `Authorization: Bearer <token>`. Handlers que precisam de login
/// simplesmente pedem `CurrentUser` como parâmetro -- é o equivalente ao
/// atributo `[Authorize]` + `User.FindFirst(...)` do ASP.NET, mas
/// verificado em tempo de compilação: uma rota que esquece o parâmetro
/// simplesmente não tem acesso ao usuário, não existe "esquecer o
/// atributo" e vazar um endpoint sem querer.
pub struct CurrentUser {
    pub id: i64,
    pub email: String,
    pub nome: String,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let app_state = AppState::from_ref(state);
        let header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let token = header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;
        let claims = decode_token(&app_state.jwt_secret, token)?;
        let id = claims
            .sub
            .parse::<i64>()
            .map_err(|_| AppError::Unauthorized)?;

        // Token só vale se a versão da sessão ainda é a do banco (logout
        // invalida) e o usuário ainda existe.
        let ver_atual: Option<i64> = app_state
            .pool
            .get()?
            .query_row(
                "SELECT sessao_versao FROM usuarios WHERE id = ?1",
                [id],
                |r| r.get(0),
            )
            .optional()?;
        if ver_atual != Some(claims.ver) {
            return Err(AppError::Unauthorized);
        }

        Ok(CurrentUser {
            id,
            email: claims.email,
            nome: claims.nome,
        })
    }
}

pub fn hash_password(password: &str) -> Result<String> {
    crate::crypto::hash_master_password(password).map_err(AppError::from)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    crate::crypto::verify_master_password(password, hash).map_err(AppError::from)
}

pub struct TwoFactorSetup {
    pub secret_base32: String,
    pub otpauth_uri: String,
}

pub fn setup_2fa(email: &str) -> Result<TwoFactorSetup> {
    let secret = Secret::generate_secret();
    let secret_base32 = secret.to_encoded().to_string();
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret
            .to_bytes()
            .map_err(|e| AppError::Internal(e.to_string()))?,
        Some("CofreSenhas".to_string()),
        email.to_string(),
    )
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(TwoFactorSetup {
        secret_base32,
        otpauth_uri: totp.get_url(),
    })
}

/// Verifica um código TOTP com uma janela de tolerância de ±1 passo (30s
/// antes/depois), igual ao `VerificationWindow(1, 1)` do backend original
/// -- necessário porque o relógio do celular do usuário pode estar
/// levemente dessincronizado do servidor.
pub fn verify_totp(secret_base32: &str, code: &str) -> bool {
    let Ok(secret_bytes) = Secret::Encoded(secret_base32.to_string()).to_bytes() else {
        return false;
    };
    let Ok(totp) = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret_bytes,
        None,
        "user".to_string(),
    ) else {
        return false;
    };

    let now = now_secs();
    [0i64, -1, 1].iter().any(|step_offset| {
        let ts = (now as i64 + step_offset * 30).max(0) as u64;
        totp.generate(ts) == code
    })
}

const ALFABETO_RECUPERACAO: &[u8] = b"abcdefghjkmnpqrstuvwxyz23456789";
pub const QTD_CODIGOS_RECUPERACAO: usize = 10;

/// Gera um código de recuperação de uso único no formato `xxxxx-xxxxx`
/// (10 símbolos de um alfabeto de 31 = ~50 bits).
pub fn gerar_codigo_recuperacao() -> String {
    use rand::seq::SliceRandom;
    let mut rng = rand::rngs::OsRng;
    let c: String = (0..10)
        .map(|_| *ALFABETO_RECUPERACAO.choose(&mut rng).unwrap() as char)
        .collect();
    format!("{}-{}", &c[..5], &c[5..])
}

/// Hash do código normalizado (sem hífen/espaço, minúsculo). SHA-256 basta:
/// o código já tem entropia alta e a verificação é limitada por rate limit.
pub fn hash_codigo_recuperacao(codigo: &str) -> String {
    let norm: String = codigo
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .collect::<String>()
        .to_lowercase();
    Sha256::digest(norm.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
