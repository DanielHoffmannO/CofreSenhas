use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;

use crate::error::{Result, VaultError};

/// Gera o hash da master password para ser guardado no banco.
/// O salt é gerado aleatoriamente e fica embutido na string retornada
/// (formato PHC), então não precisamos armazená-lo separadamente.
pub fn hash_master_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| VaultError::Crypto(e.to_string()))?;
    Ok(hash.to_string())
}

/// Verifica se `password` corresponde ao `stored_hash` previamente gerado
/// por `hash_master_password`. Não retorna a senha nem o hash decifrado
/// -- Argon2 é uma via de mão única.
pub fn verify_master_password(password: &str, stored_hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(stored_hash).map_err(|e| VaultError::Crypto(e.to_string()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Deriva uma chave simétrica de 32 bytes a partir da master password e de
/// um salt específico para a cifra (diferente do salt embutido no hash de
/// verificação acima -- propósitos diferentes, salts diferentes).
pub fn derive_encryption_key(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| VaultError::Crypto(e.to_string()))?;
    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_roundtrip() {
        let hash = hash_master_password("uma-senha-forte").unwrap();
        assert!(verify_master_password("uma-senha-forte", &hash).unwrap());
        assert!(!verify_master_password("senha-errada", &hash).unwrap());
    }

    #[test]
    fn derive_key_is_deterministic_for_same_salt() {
        let salt = b"01234567890123456789012345678901";
        let k1 = derive_encryption_key("minha-master", salt).unwrap();
        let k2 = derive_encryption_key("minha-master", salt).unwrap();
        assert_eq!(k1, k2);
    }
}
