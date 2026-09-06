mod sqlite;

pub use sqlite::SqliteStorage;

use crate::error::Result;
use crate::models::EncryptedCredential;

/// Registro de "quem é o dono do cofre": o hash da master password (para
/// verificação no login) e o salt usado para derivar a chave de cifra das
/// credenciais (ver `crypto::derive_encryption_key`).
pub struct MasterRecord {
    pub password_hash: String,
    pub key_salt: Vec<u8>,
}

/// Abstração do backend de persistência. `crypto` e `cli` só conhecem esta
/// trait -- nunca `rusqlite` diretamente -- então trocar o backend (JSON,
/// Postgres, um mock em memória para testes) significa escrever um novo
/// `impl VaultStorage for Xxx`, sem tocar em mais nada.
pub trait VaultStorage {
    /// Cria o schema, se ainda não existir. Idempotente.
    fn init(&self) -> Result<()>;

    fn save_master_record(&self, record: &MasterRecord) -> Result<()>;
    fn load_master_record(&self) -> Result<Option<MasterRecord>>;

    fn add_credential(&self, credential: &EncryptedCredential) -> Result<()>;
    fn get_credential(&self, service: &str) -> Result<EncryptedCredential>;
    fn list_credentials(&self) -> Result<Vec<EncryptedCredential>>;
    fn update_credential(&self, credential: &EncryptedCredential) -> Result<()>;
    fn delete_credential(&self, service: &str) -> Result<()>;
}
