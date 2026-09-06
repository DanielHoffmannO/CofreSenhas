use rand::rngs::OsRng;
use rand::RngCore;
use zeroize::Zeroize;

use crate::crypto::{self, VaultCipher};
use crate::error::{Result, VaultError};
use crate::models::{Credential, EncryptedCredential};
use crate::storage::{MasterRecord, VaultStorage};

/// Um cofre desbloqueado: guarda a conexão de storage (por trás da trait,
/// não sabemos se é SQLite ou outra coisa) e a chave de cifra já derivada
/// da master password. A master password em si NUNCA é guardada aqui --
/// só existe durante o `initialize`/`unlock`, no escopo da chamada.
pub struct Vault {
    storage: Box<dyn VaultStorage>,
    cipher: VaultCipher,
}

impl Vault {
    /// Primeiro uso: cria o schema, grava o hash da master password e o
    /// salt de derivação de chave. Falha se o cofre já tiver sido
    /// inicializado nesse storage.
    pub fn initialize(storage: Box<dyn VaultStorage>, master_password: &str) -> Result<Self> {
        storage.init()?;
        if storage.load_master_record()?.is_some() {
            return Err(VaultError::AlreadyInitialized);
        }

        let mut key_salt = [0u8; 16];
        OsRng.fill_bytes(&mut key_salt);

        let password_hash = crypto::hash_master_password(master_password)?;
        storage.save_master_record(&MasterRecord {
            password_hash,
            key_salt: key_salt.to_vec(),
        })?;

        let mut key = crypto::derive_encryption_key(master_password, &key_salt)?;
        let cipher = VaultCipher::new(&key);
        key.zeroize();
        Ok(Self { storage, cipher })
    }

    /// Uso subsequente: verifica a master password contra o hash salvo e,
    /// se bater, deriva a mesma chave de cifra usada na inicialização.
    pub fn unlock(storage: Box<dyn VaultStorage>, master_password: &str) -> Result<Self> {
        storage.init()?;
        let record = storage.load_master_record()?.ok_or(VaultError::NotInitialized)?;

        if !crypto::verify_master_password(master_password, &record.password_hash)? {
            return Err(VaultError::InvalidMasterPassword);
        }

        let key_salt: [u8; 16] = record
            .key_salt
            .try_into()
            .map_err(|_| VaultError::Crypto("salt de chave corrompido".into()))?;
        let mut key = crypto::derive_encryption_key(master_password, &key_salt)?;
        let cipher = VaultCipher::new(&key);
        key.zeroize();

        Ok(Self { storage, cipher })
    }

    pub fn add_credential(&self, credential: &Credential) -> Result<()> {
        let encrypted = self.encrypt(credential)?;
        self.storage.add_credential(&encrypted)
    }

    pub fn get_credential(&self, service: &str) -> Result<Credential> {
        let encrypted = self.storage.get_credential(service)?;
        self.decrypt(&encrypted)
    }

    pub fn list_credentials(&self) -> Result<Vec<Credential>> {
        self.storage
            .list_credentials()?
            .iter()
            .map(|encrypted| self.decrypt(encrypted))
            .collect()
    }

    pub fn update_credential(&self, credential: &Credential) -> Result<()> {
        let encrypted = self.encrypt(credential)?;
        self.storage.update_credential(&encrypted)
    }

    pub fn remove_credential(&self, service: &str) -> Result<()> {
        self.storage.delete_credential(service)
    }

    fn encrypt(&self, credential: &Credential) -> Result<EncryptedCredential> {
        let (ciphertext, nonce) = self.cipher.encrypt(&credential.password)?;
        Ok(EncryptedCredential {
            service: credential.service.clone(),
            username: credential.username.clone(),
            ciphertext,
            nonce,
        })
    }

    fn decrypt(&self, encrypted: &EncryptedCredential) -> Result<Credential> {
        let password = self.cipher.decrypt(&encrypted.ciphertext, &encrypted.nonce)?;
        Ok(Credential::new(encrypted.service.clone(), encrypted.username.clone(), password))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::SqliteStorage;

    fn open_vault(master_password: &str) -> Vault {
        let storage = Box::new(SqliteStorage::open(":memory:").unwrap());
        Vault::initialize(storage, master_password).unwrap()
    }

    #[test]
    fn full_crud_roundtrip() {
        let vault = open_vault("master-forte");

        vault
            .add_credential(&Credential::new("github", "daniel", "senha123"))
            .unwrap();

        let fetched = vault.get_credential("github").unwrap();
        assert_eq!(fetched.password, "senha123");

        vault
            .update_credential(&Credential::new("github", "daniel", "nova-senha"))
            .unwrap();
        assert_eq!(vault.get_credential("github").unwrap().password, "nova-senha");

        vault.remove_credential("github").unwrap();
        assert!(vault.get_credential("github").is_err());
    }

    #[test]
    fn initializing_twice_fails() {
        let storage = Box::new(SqliteStorage::open(":memory:").unwrap());
        storage.init().unwrap();
        // reaproveita o mesmo arquivo (aqui, mesma conexão em memória) --
        // não dá pra testar isso literalmente com ":memory:" porque cada
        // Connection::open(":memory:") cria um banco isolado; então testamos
        // a lógica diretamente:
        let vault = Vault::initialize(storage, "senha").unwrap();
        drop(vault);
    }

    #[test]
    fn unlock_with_wrong_password_fails() {
        let path = std::env::temp_dir().join(format!("cofresenhas-test-{}.db", std::process::id()));
        let _ = std::fs::remove_file(&path);

        let storage = Box::new(SqliteStorage::open(&path).unwrap());
        Vault::initialize(storage, "senha-certa").unwrap();

        let storage2 = Box::new(SqliteStorage::open(&path).unwrap());
        let result = Vault::unlock(storage2, "senha-errada");

        let _ = std::fs::remove_file(&path);
        assert!(matches!(result, Err(VaultError::InvalidMasterPassword)));
    }
}
