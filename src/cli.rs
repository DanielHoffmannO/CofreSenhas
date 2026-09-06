use std::path::PathBuf;

use clap::{Parser, Subcommand};
use zeroize::Zeroizing;

use crate::error::Result;
use crate::models::Credential;
use crate::storage::SqliteStorage;
use crate::vault::Vault;

#[derive(Parser)]
#[command(name = "cofresenhas", about = "Gerenciador de senhas em Rust", version)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Caminho do arquivo do cofre (SQLite)
    #[arg(long, global = true, default_value = "cofre.db")]
    vault_path: PathBuf,
}

#[derive(Subcommand)]
enum Command {
    /// Cria um novo cofre neste arquivo, protegido por uma master password
    Init,
    /// Cadastra uma nova credencial
    Add { service: String, username: String },
    /// Busca uma credencial pelo nome do serviço
    Get { service: String },
    /// Lista todos os serviços cadastrados
    List,
    /// Atualiza usuário/senha de uma credencial existente
    Update { service: String, username: String },
    /// Remove uma credencial
    Remove { service: String },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let storage = Box::new(SqliteStorage::open(&cli.vault_path)?);

    match cli.command {
        Command::Init => {
            let password = prompt_new_master_password()?;
            Vault::initialize(storage, &password)?;
            println!("Cofre criado em {}", cli.vault_path.display());
        }
        Command::Add { service, username } => {
            let vault = unlock(storage)?;
            let password = prompt_password("Senha a guardar: ")?;
            vault.add_credential(&Credential::new(service.clone(), username, password.as_str()))?;
            println!("Credencial '{service}' cadastrada.");
        }
        Command::Get { service } => {
            let vault = unlock(storage)?;
            let credential = vault.get_credential(&service)?;
            println!("serviço:  {}", credential.service);
            println!("usuário:  {}", credential.username);
            println!("senha:    {}", credential.password);
        }
        Command::List => {
            let vault = unlock(storage)?;
            let credentials = vault.list_credentials()?;
            if credentials.is_empty() {
                println!("Nenhuma credencial cadastrada.");
            }
            for credential in &credentials {
                println!("{}  ({})", credential.service, credential.username);
            }
        }
        Command::Update { service, username } => {
            let vault = unlock(storage)?;
            let password = prompt_password("Nova senha: ")?;
            vault.update_credential(&Credential::new(service.clone(), username, password.as_str()))?;
            println!("Credencial '{service}' atualizada.");
        }
        Command::Remove { service } => {
            let vault = unlock(storage)?;
            vault.remove_credential(&service)?;
            println!("Credencial '{service}' removida.");
        }
    }

    Ok(())
}

fn unlock(storage: Box<SqliteStorage>) -> Result<Vault> {
    let password = prompt_password("Master password: ")?;
    Vault::unlock(storage, &password)
}

fn prompt_password(label: &str) -> Result<Zeroizing<String>> {
    Ok(Zeroizing::new(rpassword::prompt_password(label)?))
}

fn prompt_new_master_password() -> Result<Zeroizing<String>> {
    let password = prompt_password("Defina a master password: ")?;
    let confirmation = prompt_password("Confirme a master password: ")?;
    if *password != *confirmation {
        return Err(crate::error::VaultError::Crypto(
            "as senhas informadas não coincidem".into(),
        ));
    }
    Ok(password)
}
