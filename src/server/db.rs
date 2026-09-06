use r2d2_sqlite::SqliteConnectionManager;

use crate::error::{Result, VaultError};

pub type DbPool = r2d2::Pool<SqliteConnectionManager>;

pub fn init_pool(path: &str) -> Result<DbPool> {
    let manager = SqliteConnectionManager::file(path);
    let pool = r2d2::Pool::new(manager).map_err(|e| VaultError::Crypto(e.to_string()))?;
    run_migrations(&pool)?;
    Ok(pool)
}

fn run_migrations(pool: &DbPool) -> Result<()> {
    let conn = pool.get().map_err(|e| VaultError::Crypto(e.to_string()))?;
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS usuarios (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            senha_hash TEXT NOT NULL,
            criado_em TEXT NOT NULL,
            two_factor_enabled INTEGER NOT NULL DEFAULT 0,
            two_factor_secret TEXT,
            master_password_hash TEXT,
            master_password_salt BLOB,
            encryption_salt BLOB NOT NULL
        );

        CREATE TABLE IF NOT EXISTS senhas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            usuario_id INTEGER NOT NULL REFERENCES usuarios(id) ON DELETE CASCADE,
            titulo TEXT NOT NULL,
            login TEXT NOT NULL,
            senha_ciphertext BLOB NOT NULL,
            senha_nonce BLOB NOT NULL,
            url TEXT,
            notas TEXT,
            categoria TEXT NOT NULL DEFAULT 'Pessoal',
            criado_em TEXT NOT NULL,
            atualizado_em TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS senha_versoes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            senha_id INTEGER NOT NULL REFERENCES senhas(id) ON DELETE CASCADE,
            titulo TEXT NOT NULL,
            login TEXT NOT NULL,
            senha_ciphertext BLOB NOT NULL,
            senha_nonce BLOB NOT NULL,
            alterado_em TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS historico_geracao (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            usuario_id INTEGER NOT NULL REFERENCES usuarios(id) ON DELETE CASCADE,
            prompt TEXT NOT NULL,
            senha_gerada TEXT NOT NULL,
            forca_senha TEXT NOT NULL,
            criado_em TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS audit_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            usuario_id INTEGER NOT NULL REFERENCES usuarios(id) ON DELETE CASCADE,
            senha_id INTEGER NOT NULL,
            acao TEXT NOT NULL,
            data_hora TEXT NOT NULL
        );
        ",
    )?;
    Ok(())
}
