use chacha20poly1305::aead::{Aead, OsRng as AeadOsRng};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, KeyInit};

use crate::error::{Result, VaultError};

/// Envolve uma chave já derivada e sabe cifrar/decifrar senhas individuais.
/// Cada `Vault` (sessão desbloqueada) tem um `VaultCipher`; ele não guarda
/// a master password em nenhum momento, só a chave derivada dela.
pub struct VaultCipher {
    cipher: ChaCha20Poly1305,
}

impl VaultCipher {
    pub fn new(key_bytes: &[u8; 32]) -> Self {
        let key = Key::from_slice(key_bytes);
        Self {
            cipher: ChaCha20Poly1305::new(key),
        }
    }

    /// Cifra `plaintext` e devolve (ciphertext, nonce). O nonce é gerado
    /// aleatoriamente a cada chamada -- nunca reutilizamos um nonce com a
    /// mesma chave, isso quebraria a segurança do ChaCha20-Poly1305.
    pub fn encrypt(&self, plaintext: &str) -> Result<(Vec<u8>, Vec<u8>)> {
        let nonce = ChaCha20Poly1305::generate_nonce(&mut AeadOsRng);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .map_err(|e| VaultError::Crypto(e.to_string()))?;
        Ok((ciphertext, nonce.to_vec()))
    }

    /// Decifra usando o nonce que foi salvo junto do ciphertext. Se a chave
    /// (ou seja, a master password) estiver errada, ou os dados tiverem
    /// sido adulterados, a tag de autenticação falha e devolvemos
    /// `InvalidMasterPassword` -- não dá pra distinguir os dois casos, e
    /// isso é intencional (evita vazar informação sobre qual estava errado).
    pub fn decrypt(&self, ciphertext: &[u8], nonce_bytes: &[u8]) -> Result<String> {
        let nonce = chacha20poly1305::Nonce::from_slice(nonce_bytes);
        let plaintext_bytes = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| VaultError::InvalidMasterPassword)?;
        String::from_utf8(plaintext_bytes).map_err(|e| VaultError::Crypto(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let key = [7u8; 32];
        let cipher = VaultCipher::new(&key);

        let (ciphertext, nonce) = cipher.encrypt("hunter2").unwrap();
        let decrypted = cipher.decrypt(&ciphertext, &nonce).unwrap();

        assert_eq!(decrypted, "hunter2");
    }

    #[test]
    fn wrong_key_fails_to_decrypt() {
        let cipher_a = VaultCipher::new(&[1u8; 32]);
        let cipher_b = VaultCipher::new(&[2u8; 32]);

        let (ciphertext, nonce) = cipher_a.encrypt("segredo").unwrap();
        let result = cipher_b.decrypt(&ciphertext, &nonce);

        assert!(result.is_err());
    }
}
