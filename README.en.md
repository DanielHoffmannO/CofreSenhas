🌐 [Português](README.md) | [Español](README.es.md)

# 🔐 Password Vault

[![Rust CI](https://github.com/DanielHoffmannO/CofreSenhas/actions/workflows/rust.yml/badge.svg)](https://github.com/DanielHoffmannO/CofreSenhas/actions)
![Rust](https://img.shields.io/badge/Rust-stable-000000?logo=rust)
![React](https://img.shields.io/badge/React-18-61DAFB?logo=react)
![SQLite](https://img.shields.io/badge/SQLite-embedded-003B57?logo=sqlite&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-Ready-2496ED?logo=docker&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-green)

> Personal password vault with strong encryption, 2FA authentication and a browser extension -- zero-knowledge model.
>
> Backend rewritten from scratch in idiomatic Rust (used to be .NET/ASP.NET Core), keeping the same React frontend and the same API contract.

## Screenshots

| Login | Dashboard | Generator |
|:-----:|:---------:|:---------:|
| ![Login](docs/screenshots/Login.png) | ![Dashboard](docs/screenshots/Dashboard.png) | ![Generator](docs/screenshots/Gerador.png) |

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Front-end | React 18 + TypeScript + Tailwind CSS + Vite |
| Back-end | Rust + axum (Web API) |
| Database | SQLite (embedded, via rusqlite) |
| Auth | JWT (Bearer Token) + 2FA (TOTP) |
| Encryption | ChaCha20-Poly1305 (passwords) + Argon2 (account password) |
| Infra | Docker Compose |
| Extension | Browser Extension (Firefox / LibreWolf) |

## How to Run

```bash
cp .env.example .env   # adjust ports/secrets if needed
docker-compose up --build
```

| Service | URL |
|---------|-----|
| Front-end | http://localhost:8080 |
| API | http://localhost:5000/api |
| Health check | http://localhost:5000/health |

> ⚠️ **`JWT_SECRET` and `ENCRYPTION_SECRET` in `.env` must be strong, unique values** (e.g. `openssl rand -base64 48`) before storing real passwords -- the defaults baked into the code are public. `ENCRYPTION_SECRET` derives the key that encrypts passwords in the database: **changing it after data has already been saved makes that data permanently unreadable.** Keep it somewhere safe (a password manager, a vault) -- losing the `.env` without a backup means losing the whole vault.

### Backup

The database is a single SQLite file (`cofresenhas.db`, or the `api-data` volume in Docker). There is no automatic backup. Recommended:

```bash
# Consistent backup even while the server is running (SQLite online backup)
sqlite3 cofresenhas.db ".backup cofresenhas-backup-$(date +%Y%m%d).db"
```

Run this periodically (cron/systemd timer) and store the copy off-machine.

### Without Docker (dev)

```bash
# API (port 5000 by default)
cargo run --bin server

# Front-end
cd frontend && npm install && npm run dev
```

The front-end (`vite dev`, port 3000) already has a proxy configured for `/api` -> `http://localhost:5000`.

### Local CLI (bonus)

This same Rust binary also exposes a standalone, single-user CLI that doesn't need the web server -- handy for personal terminal use:

```bash
cargo run --bin cofresenhas -- init
cargo run --bin cofresenhas -- add github my-username
```

See [`src/cli.rs`](src/cli.rs) and the `crypto`/`storage`/`vault` modules for details.

## Features

- ✅ Login and registration with JWT + 2FA authentication (TOTP)
- ✅ Passwords encrypted (ChaCha20-Poly1305) in the database -- zero-knowledge model
- ✅ Full CRUD for passwords
- ✅ Configurable generator (length, uppercase, numbers, special chars)
- ✅ Strength indicator (Weak -> Very Strong)
- ✅ Browser extension (Firefox / LibreWolf) with automatic fill suggestions
- ✅ Export/Import (JSON and CSV, with duplicate detection on import)
- ✅ One-click copy + show/hide toggle
- ✅ Responsive dark mode interface
- ✅ Health check

> Login rate limiting, a separate master password, and password version history existed in the original .NET backend or in earlier versions of this one; they were deliberately left out of the Rust rewrite because they didn't earn their keep (maintenance cost with no real use) -- see the project's philosophy.

## Architecture

```
src/
├── lib.rs           ← shared library entry point
├── bin/
│   ├── server.rs     ← web server entry point (axum)
│   └── cofresenhas.rs ← standalone CLI entry point
├── crypto/           ← Argon2 (hashing) + ChaCha20-Poly1305 (encryption) -- used by both CLI and server
├── storage/, vault.rs, cli.rs  ← local single-user CLI
└── server/           ← multi-user web API
    ├── db.rs          ← SQLite pool + schema
    ├── auth.rs        ← JWT, password hashing, TOTP
    ├── models.rs       ← DTOs (same JSON contract as the frontend)
    └── handlers/       ← routes: auth, passwords, generator
frontend/
└── React + TypeScript + Tailwind + Vite
extension/
└── Browser Extension (Manifest V2 -- Firefox / LibreWolf)
```

## Tests

```bash
cargo test
```

## License

This project is licensed under [MIT](LICENSE).
