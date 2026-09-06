use thiserror::Error;

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("credencial não encontrada para o serviço '{0}'")]
    CredentialNotFound(String),

    #[error("já existe uma credencial cadastrada para o serviço '{0}'")]
    CredentialAlreadyExists(String),

    #[error("master password incorreta")]
    InvalidMasterPassword,

    #[error("o cofre já foi inicializado nesse arquivo")]
    AlreadyInitialized,

    #[error("o cofre ainda não foi inicializado -- rode o comando `init` primeiro")]
    NotInitialized,

    #[error("falha de criptografia: {0}")]
    Crypto(String),

    #[error("erro de armazenamento: {0}")]
    Storage(#[from] rusqlite::Error),

    #[error("erro de I/O: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, VaultError>;
