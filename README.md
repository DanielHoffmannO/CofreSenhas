[>] [English](README.en.md) | [Espanol](README.es.md)

# {*} Cofre de Senhas

[![.NET CI](https://github.com/DanielHoffmannO/CofreSenhas/actions/workflows/dotnet.yml/badge.svg)](https://github.com/DanielHoffmannO/CofreSenhas/actions)
![.NET](https://img.shields.io/badge/.NET-9.0-512BD4?logo=dotnet)
![React](https://img.shields.io/badge/React-18-61DAFB?logo=react)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-16-4169E1?logo=postgresql&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-Ready-2496ED?logo=docker&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-green)

> Gerenciador de senhas pessoal com criptografia AES-256, autenticacao 2FA e extensao para browser -- modelo zero-knowledge.

## [~] Screenshots

| Login | Dashboard | Gerador |
|:-----:|:---------:|:-------:|
| ![Login](docs/screenshots/Login.png) | ![Dashboard](docs/screenshots/Dashboard.png) | ![Gerador](docs/screenshots/Gerador.png) |

## {=} Tech Stack

| Camada | Tecnologia |
|--------|-----------|
| Front-end | React 18 + TypeScript + Tailwind CSS + Vite |
| Back-end | .NET 9 / ASP.NET Core Web API |
| Banco | PostgreSQL 16 |
| Auth | JWT (Bearer Token) + 2FA (TOTP) |
| Criptografia | AES-256 + PBKDF2 Key Derivation (100k iteracoes) |
| Infra | Docker Compose |
| Extensao | Browser Extension (Firefox / LibreWolf) |

## [!] Como Rodar

```bash
cp .env.example .env   # ajuste as portas se necessario
docker-compose up --build
```

| Servico | URL |
|---------|-----|
| Front-end | http://localhost:8080 |
| API (Swagger) | http://localhost:5000/swagger |

### Sem Docker (dev)

```bash
# API
dotnet run --project src/CofreSenhas.Api

# Front-end
cd frontend && npm install && npm run dev
```

### Dados de Teste

| Email | Senha |
|-------|-------|
| admin@cofre.com | admin123 |

## [+] Features

- {k} Login e registro com JWT + Autenticacao 2FA (TOTP)
- {#} Senhas criptografadas (AES-256) no banco -- modelo zero-knowledge
- {%} Master Password com Key Derivation (PBKDF2 100k iteracoes)
- [w] CRUD completo de senhas com historico de versoes
- [*] Gerador configuravel (tamanho, maiusculas, numeros, especiais)
- [|] Indicador de forca (Fraca -> Muito Forte)
- [x] Extensao para browser (Firefox / LibreWolf) com auto-fill
- [^] Export/Import (JSON e CSV)
- [>] Copiar com 1 clique + Mostrar/ocultar
- [~] Interface dark mode responsiva
- [!] Rate Limiting (anti brute-force) + Auditoria de acessos
- [<3] Health Checks

## {/} Arquitetura

```
src/
+-- CofreSenhas.Domain        <- Entidades, DTOs, Interfaces, Enums
+-- CofreSenhas.Service       <- Regras de negocio (Auth, Senhas, Gerador)
+-- CofreSenhas.Persistence   <- EF Core + PostgreSQL, Repositories
+-- CofreSenhas.Api           <- Controllers, JWT, Swagger
frontend/
+-- React + TypeScript + Tailwind + Vite
extension/
+-- Browser Extension (Manifest V2 -- Firefox / LibreWolf)
tests/
+-- CofreSenhas.Tests         <- xUnit (Auth, Senhas, Gerador)
```

## [?] Testes

```bash
dotnet test
```

## [$] Licenca

Este projeto esta sob a licenca [MIT](LICENSE).
