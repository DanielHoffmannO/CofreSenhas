//! Snapshot consistente do banco SQLite (funciona com o servidor rodando).
//! Uso: `backup_db <banco> <saida>`. Roda dentro do container, onde não há
//! o CLI `sqlite3`: `docker compose exec api backup_db /data/cofresenhas.db /tmp/b.db`.
use rusqlite::Connection;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("uso: backup_db <banco> <saida>");
        std::process::exit(2);
    }
    let conn = Connection::open(&args[1]).expect("falha ao abrir o banco");
    if std::path::Path::new(&args[2]).exists() {
        eprintln!("{} já existe; recuso sobrescrever", args[2]);
        std::process::exit(1);
    }
    conn.execute("VACUUM INTO ?1", [&args[2]])
        .expect("falha ao gerar o backup");
}
