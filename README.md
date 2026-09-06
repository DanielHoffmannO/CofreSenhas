[>] [English](README.en.md) | [Espanol](README.es.md)

# {*} Cofre de Senhas

[![Rust CI](https://github.com/DanielHoffmannO/CofreSenhas/actions/workflows/rust.yml/badge.svg)](https://github.com/DanielHoffmannO/CofreSenhas/actions)
![Rust](https://img.shields.io/badge/Rust-stable-000000?logo=rust)
![React](https://img.shields.io/badge/React-18-61DAFB?logo=react)
![SQLite](https://img.shields.io/badge/SQLite-embutido-003B57?logo=sqlite&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-Ready-2496ED?logo=docker&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-green)

> Gerenciador de senhas pessoal com criptografia forte, autenticacao 2FA e extensao para browser -- modelo zero-knowledge.
>
> Backend reescrito do zero em Rust idiomatico (era .NET/ASP.NET Core), mantendo o mesmo frontend React e o mesmo contrato de API.

## [~] Screenshots

| Login | Dashboard | Gerador |
|:-----:|:---------:|:-------:|
| ![Login](docs/screenshots/Login.png) | ![Dashboard](docs/screenshots/Dashboard.png) | ![Gerador](docs/screenshots/Gerador.png) |

## {=} Tech Stack

| Camada | Tecnologia |
|--------|-----------|
| Front-end | React 18 + TypeScript + Tailwind CSS + Vite |
| Back-end | Rust + axum (Web API) |
| Banco | SQLite (embutido, via rusqlite) |
| Auth | JWT (Bearer Token) + 2FA (TOTP) |
| Criptografia | ChaCha20-Poly1305 (senhas) + Argon2 (master password / senha de conta) |
| Infra | Docker Compose |
| Extensao | Browser Extension (Firefox / LibreWolf) |

## [!] Como Rodar

```bash
cp .env.example .env   # ajuste portas/segredos se necessario
docker-compose up --build
```

| Servico | URL |
|---------|-----|
| Front-end | http://localhost:8080 |
| API | http://localhost:5000/api |
| Health check | http://localhost:5000/health |

### Sem Docker (dev)

```bash
# API (porta 5000 por padrao)
cargo run --bin server

# Front-end
cd frontend && npm install && npm run dev
```

O front-end (`vite dev`, porta 3000) já tem um proxy configurado para `/api` -> `http://localhost:5000`.

### CLI local (bonus)

Este mesmo binário Rust também expõe uma CLI standalone, single-usuário, sem precisar do servidor web -- útil para uso pessoal via terminal:

```bash
cargo run --bin cofresenhas -- init
cargo run --bin cofresenhas -- add github meu-usuario
```

Veja [`src/cli.rs`](src/cli.rs) e os módulos `crypto`/`storage`/`vault` para detalhes.

## [+] Features

- {k} Login e registro com JWT + Autenticacao 2FA (TOTP)
- {#} Senhas criptografadas (ChaCha20-Poly1305) no banco -- modelo zero-knowledge
- {%} Master Password com Argon2 Key Derivation
- [w] CRUD completo de senhas com historico de versoes
- [*] Gerador configuravel (tamanho, maiusculas, numeros, especiais)
- [|] Indicador de forca (Fraca -> Muito Forte)
- [x] Extensao para browser (Firefox / LibreWolf) com auto-fill
- [^] Export/Import (JSON e CSV)
- [>] Copiar com 1 clique + Mostrar/ocultar
- [~] Interface dark mode responsiva
- [<3] Health check

> Nota: o rate limiting de login (anti brute-force) e a auditoria de acessos existiam no backend .NET original e ainda não foram portados para o Rust.

## {/} Arquitetura

```
src/
+-- lib.rs          <- ponto de entrada da lib compartilhada
+-- bin/
|   +-- server.rs    <- entrada do servidor web (axum)
|   +-- cofresenhas.rs <- entrada da CLI standalone
+-- crypto/          <- Argon2 (hash) + ChaCha20-Poly1305 (cifra) -- usado por CLI e server
+-- storage/, vault.rs, cli.rs  <- CLI local single-usuário
+-- server/          <- API web multiusuário
|   +-- db.rs         <- pool SQLite + schema
|   +-- auth.rs       <- JWT, hash de senha, TOTP
|   +-- models.rs     <- DTOs (mesmo contrato JSON do frontend)
|   +-- handlers/     <- rotas: auth, senhas, gerador, audit
frontend/
+-- React + TypeScript + Tailwind + Vite
extension/
+-- Browser Extension (Manifest V2 -- Firefox / LibreWolf)
```

## [?] Testes

```bash
cargo test
```

## [$] Licenca

Este projeto esta sob a licenca [MIT](LICENSE).
