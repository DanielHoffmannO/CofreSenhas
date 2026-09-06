use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
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
}

pub fn generate_token(secret: &str, user_id: i64, email: &str, nome: &str) -> Result<String> {
    let exp = now_secs() + 8 * 3600; // expira em 8h, igual ao original
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        nome: nome.to_string(),
        exp: exp as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|e| AppError::Internal(format!("falha ao gerar token: {e}")))
}

fn decode_token(secret: &str, token: &str) -> Result<Claims> {
    decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized)
}

fn now_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
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

        let token = header.strip_prefix("Bearer ").ok_or(AppError::Unauthorized)?;
        let claims = decode_token(&app_state.jwt_secret, token)?;
        let id = claims.sub.parse::<i64>().map_err(|_| AppError::Unauthorized)?;

        Ok(CurrentUser { id, email: claims.email, nome: claims.nome })
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
        secret.to_bytes().map_err(|e| AppError::Internal(e.to_string()))?,
        Some("CofreSenhas".to_string()),
        email.to_string(),
    )
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(TwoFactorSetup { secret_base32, otpauth_uri: totp.get_url() })
}

/// Verifica um código TOTP com uma janela de tolerância de ±1 passo (30s
/// antes/depois), igual ao `VerificationWindow(1, 1)` do backend original
/// -- necessário porque o relógio do celular do usuário pode estar
/// levemente dessincronizado do servidor.
pub fn verify_totp(secret_base32: &str, code: &str) -> bool {
    let Ok(secret_bytes) = Secret::Encoded(secret_base32.to_string()).to_bytes() else {
        return false;
    };
    let Ok(totp) = TOTP::new(Algorithm::SHA1, 6, 1, 30, secret_bytes, None, "user".to_string()) else {
        return false;
    };

    let now = now_secs();
    [0i64, -1, 1].iter().any(|step_offset| {
        let ts = (now as i64 + step_offset * 30).max(0) as u64;
        totp.generate(ts) == code
    })
}
