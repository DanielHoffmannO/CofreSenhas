//! Biblioteca compartilhada entre os dois binários do projeto:
//! `cofresenhas` (a CLI local, single-vault) e `server` (a API web
//! multiusuário que serve o frontend React). Ambos reaproveitam
//! `crypto`, `error` e `models`; `storage`/`vault` são específicos da
//! CLI, e `server` é específico da API.

pub mod cli;
pub mod crypto;
pub mod error;
pub mod models;
pub mod server;
pub mod storage;
pub mod vault;
