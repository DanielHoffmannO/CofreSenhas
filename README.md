🌐 [English](README.en.md) | [Español](README.es.md)

# 🔐 Cofre de Senhas

[![.NET CI](https://github.com/DanielHoffmannO/CofreSenhas/actions/workflows/dotnet.yml/badge.svg)](https://github.com/DanielHoffmannO/CofreSenhas/actions)
[![codecov](https://codecov.io/gh/DanielHoffmannO/CofreSenhas/branch/main/graph/badge.svg)](https://codecov.io/gh/DanielHoffmannO/CofreSenhas)

Cofre de senhas com gerador inteligente, indicador de força e criptografia AES-256.

## Screenshots

| Login | Dashboard | Gerador |
|-------|-----------|---------|
| ![Login](docs/screenshots/login.png) | ![Dashboard](docs/screenshots/dashboard.png) | ![Gerador](docs/screenshots/gerador.png) |

## Tech Stack

| Camada | Tecnologia |
|--------|-----------|
| Front-end | React 18 + TypeScript + Tailwind CSS + Vite |
| Back-end | .NET 9 / ASP.NET Core Web API |
| Banco | PostgreSQL 16 |
| Auth | JWT (Bearer Token) |
| Criptografia | AES-256 (senhas armazenadas criptografadas) |
| Infra | Docker Compose |

## Como Rodar

```bash
docker-compose up --build
```

- **Front-end:** http://localhost:8080
- **API (Swagger):** http://localhost:5000/swagger

## Sem Docker (dev)

```bash
# API
dotnet run --project src/CofreSenhas.Api

# Front-end
cd frontend
npm install
npm run dev
```

## Dados de Teste

- **Email:** admin@cofre.com
- **Senha:** admin123

## Features

- ✅ Login e registro com JWT
- ✅ CRUD completo de senhas
- ✅ Senhas criptografadas (AES-256) no banco
- ✅ Gerador de senhas configurável (tamanho, maiúsculas, números, especiais)
- ✅ Indicador de força (Fraca, Média, Forte, Muito Forte)
- ✅ Copiar com 1 clique
- ✅ Mostrar/ocultar senha
- ✅ Interface dark mode responsiva
- ✅ Rate Limiting (anti brute-force)
- ✅ Health Checks

## Arquitetura

```
src/
├── CofreSenhas.Domain        ← Entidades, DTOs, Interfaces, Enums
├── CofreSenhas.Service       ← Regras de negócio (Auth, Senhas, Gerador)
├── CofreSenhas.Persistence   ← EF Core + PostgreSQL, Repositories
└── CofreSenhas.Api           ← Controllers, JWT, Swagger
frontend/
└── React + TypeScript + Tailwind + Vite
tests/
└── CofreSenhas.Tests         ← xUnit
```

## Testes

```bash
dotnet test
```

## Autor

Daniel Hoffmann
