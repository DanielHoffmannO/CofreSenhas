use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;

use super::{MasterRecord, VaultStorage};
use crate::error::{Result, VaultError};
use crate::models::EncryptedCredential;

pub struct SqliteStorage {
    conn: Connection,
}

impl SqliteStorage {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }
}

impl VaultStorage for SqliteStorage {
    fn init(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS master (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                password_hash TEXT NOT NULL,
                key_salt BLOB NOT NULL
            );
            CREATE TABLE IF NOT EXISTS credentials (
                service TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                ciphertext BLOB NOT NULL,
                nonce BLOB NOT NULL
            );",
        )?;
        Ok(())
    }

    fn save_master_record(&self, record: &MasterRecord) -> Result<()> {
        self.conn.execute(
            "INSERT INTO master (id, password_hash, key_salt) VALUES (1, ?1, ?2)
             ON CONFLICT(id) DO UPDATE SET password_hash = excluded.password_hash, key_salt = excluded.key_salt",
            params![record.password_hash, record.key_salt],
        )?;
        Ok(())
    }

    fn load_master_record(&self) -> Result<Option<MasterRecord>> {
        let record = self
            .conn
            .query_row(
                "SELECT password_hash, key_salt FROM master WHERE id = 1",
                [],
                |row| {
                    Ok(MasterRecord {
                        password_hash: row.get(0)?,
                        key_salt: row.get(1)?,
                    })
                },
            )
            .optional()?;
        Ok(record)
    }

    fn add_credential(&self, credential: &EncryptedCredential) -> Result<()> {
        let affected = self.conn.execute(
            "INSERT OR IGNORE INTO credentials (service, username, ciphertext, nonce) VALUES (?1, ?2, ?3, ?4)",
            params![
                credential.service,
                credential.username,
                credential.ciphertext,
                credential.nonce
            ],
        )?;
        if affected == 0 {
            return Err(VaultError::CredentialAlreadyExists(credential.service.clone()));
        }
        Ok(())
    }

    fn get_credential(&self, service: &str) -> Result<EncryptedCredential> {
        self.conn
            .query_row(
                "SELECT service, username, ciphertext, nonce FROM credentials WHERE service = ?1",
                params![service],
                Self::row_to_credential,
            )
            .optional()?
            .ok_or_else(|| VaultError::CredentialNotFound(service.to_string()))
    }

    fn list_credentials(&self) -> Result<Vec<EncryptedCredential>> {
        let mut stmt = self
            .conn
            .prepare("SELECT service, username, ciphertext, nonce FROM credentials ORDER BY service")?;
        let rows = stmt.query_map([], Self::row_to_credential)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
            .map_err(VaultError::from)
    }

    fn update_credential(&self, credential: &EncryptedCredential) -> Result<()> {
        let affected = self.conn.execute(
            "UPDATE credentials SET username = ?2, ciphertext = ?3, nonce = ?4 WHERE service = ?1",
            params![
                credential.service,
                credential.username,
                credential.ciphertext,
                credential.nonce
            ],
        )?;
        if affected == 0 {
            return Err(VaultError::CredentialNotFound(credential.service.clone()));
        }
        Ok(())
    }

    fn delete_credential(&self, service: &str) -> Result<()> {
        let affected = self
            .conn
            .execute("DELETE FROM credentials WHERE service = ?1", params![service])?;
        if affected == 0 {
            return Err(VaultError::CredentialNotFound(service.to_string()));
        }
        Ok(())
    }
}

impl SqliteStorage {
    fn row_to_credential(row: &rusqlite::Row) -> rusqlite::Result<EncryptedCredential> {
        Ok(EncryptedCredential {
            service: row.get(0)?,
            username: row.get(1)?,
            ciphertext: row.get(2)?,
            nonce: row.get(3)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> SqliteStorage {
        let storage = SqliteStorage::open(":memory:").unwrap();
        storage.init().unwrap();
        storage
    }

    fn sample_credential(service: &str) -> EncryptedCredential {
        EncryptedCredential {
            service: service.to_string(),
            username: "daniel".to_string(),
            ciphertext: vec![1, 2, 3],
            nonce: vec![4, 5, 6],
        }
    }

    #[test]
    fn add_and_get_credential() {
        let storage = setup();
        storage.add_credential(&sample_credential("github")).unwrap();

        let fetched = storage.get_credential("github").unwrap();
        assert_eq!(fetched.username, "daniel");
    }

    #[test]
    fn adding_duplicate_service_fails() {
        let storage = setup();
        storage.add_credential(&sample_credential("github")).unwrap();

        let result = storage.add_credential(&sample_credential("github"));
        assert!(matches!(result, Err(VaultError::CredentialAlreadyExists(_))));
    }

    #[test]
    fn get_missing_credential_fails() {
        let storage = setup();
        let result = storage.get_credential("nope");
        assert!(matches!(result, Err(VaultError::CredentialNotFound(_))));
    }

    #[test]
    fn list_credentials_returns_all_sorted() {
        let storage = setup();
        storage.add_credential(&sample_credential("github")).unwrap();
        storage.add_credential(&sample_credential("aws")).unwrap();

        let all = storage.list_credentials().unwrap();
        let names: Vec<_> = all.iter().map(|c| c.service.as_str()).collect();
        assert_eq!(names, vec!["aws", "github"]);
    }

    #[test]
    fn update_and_delete_credential() {
        let storage = setup();
        storage.add_credential(&sample_credential("github")).unwrap();

        let mut updated = sample_credential("github");
        updated.username = "outro-user".to_string();
        storage.update_credential(&updated).unwrap();
        assert_eq!(storage.get_credential("github").unwrap().username, "outro-user");

        storage.delete_credential("github").unwrap();
        assert!(storage.get_credential("github").is_err());
    }

    #[test]
    fn master_record_roundtrip() {
        let storage = setup();
        assert!(storage.load_master_record().unwrap().is_none());

        let record = MasterRecord {
            password_hash: "hash123".to_string(),
            key_salt: vec![9, 9, 9],
        };
        storage.save_master_record(&record).unwrap();

        let loaded = storage.load_master_record().unwrap().unwrap();
        assert_eq!(loaded.password_hash, "hash123");
        assert_eq!(loaded.key_salt, vec![9, 9, 9]);
    }
}
