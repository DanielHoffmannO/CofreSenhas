use serde::{Deserialize, Serialize};

// ---------- Linhas de banco (não serializadas diretamente) ----------

pub struct Usuario {
    pub id: i64,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub criado_em: String,
    pub two_factor_enabled: bool,
    pub two_factor_secret: Option<String>,
    pub master_password_hash: Option<String>,
    pub encryption_salt: Vec<u8>,
}

pub struct SenhaRow {
    pub id: i64,
    pub titulo: String,
    pub login: String,
    pub senha_ciphertext: Vec<u8>,
    pub senha_nonce: Vec<u8>,
    pub url: Option<String>,
    pub notas: Option<String>,
    pub categoria: String,
    pub criado_em: String,
    pub atualizado_em: String,
}

pub struct SenhaVersaoRow {
    pub id: i64,
    pub titulo: String,
    pub login: String,
    pub senha_ciphertext: Vec<u8>,
    pub senha_nonce: Vec<u8>,
    pub alterado_em: String,
}

// ---------- DTOs de auth (contrato idêntico ao frontend React) ----------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub nome: String,
    pub email: String,
    pub senha: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub email: String,
    pub senha: String,
    pub totp_code: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_factor_required: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Setup2faResponse {
    pub secret: String,
    pub qr_code_uri: String,
}

#[derive(Deserialize)]
pub struct Verify2faRequest {
    pub code: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    pub id: i64,
    pub nome: String,
    pub email: String,
    pub criado_em: String,
    pub two_factor_enabled: bool,
    pub master_password_configured: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    pub senha_atual: String,
    pub nova_senha: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupMasterPasswordRequest {
    pub master_password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyMasterPasswordRequest {
    pub master_password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MasterPasswordStatusResponse {
    pub is_configured: bool,
}

// ---------- DTOs de senhas ----------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CriarSenhaRequest {
    pub titulo: String,
    pub login: String,
    pub senha: String,
    pub url: Option<String>,
    pub notas: Option<String>,
    #[serde(default = "categoria_padrao")]
    pub categoria: String,
}

fn categoria_padrao() -> String {
    "Pessoal".to_string()
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SenhaResponse {
    pub id: i64,
    pub titulo: String,
    pub login: String,
    pub senha: String,
    pub url: Option<String>,
    pub notas: Option<String>,
    pub categoria: String,
    pub criado_em: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SenhaVersaoResponse {
    pub id: i64,
    pub titulo: String,
    pub login: String,
    pub senha: String,
    pub alterado_em: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PagedResponse<T> {
    pub items: Vec<T>,
    pub page: i64,
    pub page_size: i64,
    pub total_count: i64,
    pub total_pages: i64,
}

impl<T> PagedResponse<T> {
    pub fn new(items: Vec<T>, page: i64, page_size: i64, total_count: i64) -> Self {
        let total_pages = (total_count as f64 / page_size as f64).ceil() as i64;
        Self { items, page, page_size, total_count, total_pages }
    }
}

// ---------- DTOs do gerador ----------

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GerarSenhaRequest {
    pub tamanho: usize,
    pub usar_maiusculas: bool,
    pub usar_numeros: bool,
    pub usar_especiais: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ForcaSenha {
    Fraca,
    Media,
    Forte,
    MuitoForte,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GerarSenhaResponse {
    pub senha: String,
    pub forca: ForcaSenha,
}

// ---------- DTOs de auditoria ----------

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogResponse {
    pub id: i64,
    pub acao: String,
    pub data_hora: String,
    pub titulo: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditRequest {
    pub senha_id: i64,
    pub acao: String,
}
