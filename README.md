ðŸŒ [English](README.en.md) | [EspaÃ±ol](README.es.md)

# ðŸ” Cofre de Senhas

[![.NET CI](https://github.com/DanielHoffmannO/CofreSenhas/actions/workflows/dotnet.yml/badge.svg)](https://github.com/DanielHoffmannO/CofreSenhas/actions)
![.NET](https://img.shields.io/badge/.NET-9.0-512BD4?logo=dotnet)
![React](https://img.shields.io/badge/React-18-61DAFB?logo=react)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-16-4169E1?logo=postgresql&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-Ready-2496ED?logo=docker&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-green)

> Gerenciador de senhas pessoal com criptografia AES-256, autenticaÃ§Ã£o 2FA e extensÃ£o para browser â€” modelo zero-knowledge.

## ðŸ“¸ Screenshots

| Login | Dashboard | Gerador |
|:-----:|:---------:|:-------:|
| ![Login](docs/screenshots/Login.png) | ![Dashboard](docs/screenshots/Dashboard.png) | ![Gerador](docs/screenshots/Gerador.png) |

## ðŸ› ï¸ Tech Stack

| Camada | Tecnologia |
|--------|-----------|
| Front-end | React 18 + TypeScript + Tailwind CSS + Vite |
| Back-end | .NET 9 / ASP.NET Core Web API |
| Banco | PostgreSQL 16 |
| Auth | JWT (Bearer Token) + 2FA (TOTP) |
| Criptografia | AES-256 + PBKDF2 Key Derivation (100k iteraÃ§Ãµes) |
| Infra | Docker Compose |
| ExtensÃ£o | Browser Extension (Firefox / LibreWolf) |

## ðŸš€ Como Rodar

```bash
cp .env.example .env   # ajuste as portas se necessÃ¡rio
docker-compose up --build
```

| ServiÃ§o | URL |
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

## âœ¨ Features

- ðŸ”‘ Login e registro com JWT + AutenticaÃ§Ã£o 2FA (TOTP)
- ðŸ”’ Senhas criptografadas (AES-256) no banco â€” modelo zero-knowledge
- ðŸ—ï¸ Master Password com Key Derivation (PBKDF2 100k iteraÃ§Ãµes)
- ðŸ“ CRUD completo de senhas com histÃ³rico de versÃµes
- âš¡ Gerador configurÃ¡vel (tamanho, maiÃºsculas, nÃºmeros, especiais)
- ðŸ“Š Indicador de forÃ§a (Fraca â†’ Muito Forte)
- ðŸ§© ExtensÃ£o para browser (Firefox / LibreWolf) com auto-fill
- ðŸ“¤ Export/Import (JSON e CSV)
- ðŸ“‹ Copiar com 1 clique + Mostrar/ocultar
- ðŸŒ™ Interface dark mode responsiva
- ðŸ›¡ï¸ Rate Limiting (anti brute-force) + Auditoria de acessos
- â¤ï¸ Health Checks

## ðŸ—ï¸ Arquitetura

```
src/
â”œâ”€â”€ CofreSenhas.Domain        â† Entidades, DTOs, Interfaces, Enums
â”œâ”€â”€ CofreSenhas.Service       â† Regras de negÃ³cio (Auth, Senhas, Gerador)
â”œâ”€â”€ CofreSenhas.Persistence   â† EF Core + PostgreSQL, Repositories
â””â”€â”€ CofreSenhas.Api           â† Controllers, JWT, Swagger
frontend/
â””â”€â”€ React + TypeScript + Tailwind + Vite
extension/
â””â”€â”€ Browser Extension (Manifest V2 â€” Firefox / LibreWolf)
tests/
â””â”€â”€ CofreSenhas.Tests         â† xUnit (Auth, Senhas, Gerador)
```

## ðŸ§ª Testes

```bash
dotnet test
```

## ðŸ“„ LicenÃ§a

Este projeto estÃ¡ sob a licenÃ§a [MIT](LICENSE).

## ðŸ‘¤ Autor

**Daniel Hoffmann** â€” [LinkedIn](https://www.linkedin.com/in/daniel-hoffmann-bonicio/) Â· [GitHub](https://github.com/DanielHoffmannO)
