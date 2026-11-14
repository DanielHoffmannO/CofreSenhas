//! Ferramenta de manutenção pontual: re-cifra todas as senhas guardadas quando o
//! ENCRYPTION_SECRET muda. Necessária porque a chave de cifra é derivada dele
//! (ver `src/bin/server.rs`) -- trocar o secret sem isso torna os dados existentes
//! ilegíveis. Não faz parte do binário `server`, roda uma vez e só.
//!
//! Uso: DATABASE_PATH=... OLD_ENCRYPTION_SECRET=... NEW_ENCRYPTION_SECRET=... \
//!      ./migrate_encryption_secret
//!
//! É seguro rodar mais de uma vez contra o mesmo par (old, new): se OLD já não bate
//! com o ciphertext atual (ou seja, a migração já foi aplicada), a decifra falha e o
//! programa aborta sem alterar nada -- não há como corromper dado já migrado.

use cofresenhas_rs::crypto::{derive_encryption_key, VaultCipher};
use cofresenhas_rs::error::{Result, VaultError};
use rusqlite::{params, Connection};

fn required_env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| {
        eprintln!("Faltando variável de ambiente obrigatória: {name}");
        std::process::exit(1);
    })
}

fn main() -> Result<()> {
    let database_path = required_env("DATABASE_PATH");
    let old_secret = required_env("OLD_ENCRYPTION_SECRET");
    let new_secret = required_env("NEW_ENCRYPTION_SECRET");

    if old_secret == new_secret {
        eprintln!("OLD_ENCRYPTION_SECRET e NEW_ENCRYPTION_SECRET são iguais, nada a fazer.");
        std::process::exit(1);
    }

    let mut conn = Connection::open(&database_path)?;

    let mut usuarios: Vec<(i64, Vec<u8>)> = Vec::new();
    {
        let mut stmt = conn.prepare("SELECT id, encryption_salt FROM usuarios")?;
        let rows = stmt.query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, Vec<u8>>(1)?)))?;
        for row in rows {
            usuarios.push(row?);
        }
    }
    println!("Usuários encontrados: {}", usuarios.len());

    let mut total_migradas = 0usize;

    let tx = conn.transaction()?;
    for (usuario_id, salt) in &usuarios {
        let key_old = derive_encryption_key(&old_secret, salt)?;
        let key_new = derive_encryption_key(&new_secret, salt)?;
        let cipher_old = VaultCipher::new(&key_old);
        let cipher_new = VaultCipher::new(&key_new);

        let mut senhas: Vec<(i64, Vec<u8>, Vec<u8>)> = Vec::new();
        {
            let mut stmt = tx.prepare(
                "SELECT id, senha_ciphertext, senha_nonce FROM senhas WHERE usuario_id = ?1",
            )?;
            let rows = stmt.query_map(params![usuario_id], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, Vec<u8>>(1)?,
                    r.get::<_, Vec<u8>>(2)?,
                ))
            })?;
            for row in rows {
                senhas.push(row?);
            }
        }

        for (senha_id, ciphertext, nonce) in senhas {
            // Se isso falhar (chave antiga errada, ou já migrado antes), aborta tudo --
            // a transação nunca comita e nada no banco muda.
            let plaintext = cipher_old.decrypt(&ciphertext, &nonce).map_err(|_| {
                VaultError::Crypto(format!(
                    "falha ao decifrar senha id={senha_id} com OLD_ENCRYPTION_SECRET -- \
                     abortando sem alterar nada (talvez já tenha sido migrado antes?)"
                ))
            })?;
            let (new_ciphertext, new_nonce) = cipher_new.encrypt(&plaintext)?;
            tx.execute(
                "UPDATE senhas SET senha_ciphertext = ?1, senha_nonce = ?2 WHERE id = ?3",
                params![new_ciphertext, new_nonce, senha_id],
            )?;
            total_migradas += 1;
        }
    }
    tx.commit()?;
    println!("Re-cifradas {total_migradas} senhas. Commit feito.");

    // Validação: relê tudo do zero e confirma que decifra com a chave NOVA.
    let mut confirmadas = 0usize;
    for (usuario_id, salt) in &usuarios {
        let key_new = derive_encryption_key(&new_secret, salt)?;
        let cipher_new = VaultCipher::new(&key_new);
        let mut stmt =
            conn.prepare("SELECT senha_ciphertext, senha_nonce FROM senhas WHERE usuario_id = ?1")?;
        let rows = stmt.query_map(params![usuario_id], |r| {
            Ok((r.get::<_, Vec<u8>>(0)?, r.get::<_, Vec<u8>>(1)?))
        })?;
        for row in rows {
            let (ciphertext, nonce) = row?;
            cipher_new.decrypt(&ciphertext, &nonce)?;
            confirmadas += 1;
        }
    }
    println!("Validação: {confirmadas}/{total_migradas} decifram corretamente com a nova chave.");
    if confirmadas != total_migradas {
        eprintln!("AVISO: número de senhas validadas não bate com o número migrado!");
        std::process::exit(1);
    }
    println!("OK.");
    Ok(())
}
