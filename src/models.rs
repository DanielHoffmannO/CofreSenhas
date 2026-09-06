use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Uma credencial já descriptografada, pronta para uso/exibição.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl Credential {
    pub fn new(service: impl Into<String>, username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            username: username.into(),
            password: password.into(),
        }
    }
}

/// Quando uma `Credential` sai de escopo -- fim de função, `Vec` sendo
/// esvaziado, etc. -- sobrescrevemos os bytes da senha em vez de deixar o
/// alocador simplesmente marcar aquela memória como livre (o que a
/// deixaria intacta e legível até ser reaproveitada por outra alocação).
impl Drop for Credential {
    fn drop(&mut self) {
        self.password.zeroize();
    }
}

/// Representação de uma credencial como ela é persistida: a senha nunca
/// existe como `String` em claro fora do momento de uso — aqui ela é
/// sempre bytes cifrados, mais o nonce usado na cifragem.
#[derive(Debug, Clone)]
pub struct EncryptedCredential {
    pub service: String,
    pub username: String,
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}
