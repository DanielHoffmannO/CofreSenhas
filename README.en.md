🌐 [Português](README.md) | [Español](README.es.md)

# 🔐 Password Vault

Password vault with smart generator, strength indicator and AES-256 encryption.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Front-end | React 18 + TypeScript + Tailwind CSS + Vite |
| Back-end | .NET 9 / ASP.NET Core Web API |
| Database | PostgreSQL 16 |
| Auth | JWT (Bearer Token) |
| Encryption | AES-256 (passwords stored encrypted) |
| Infra | Docker Compose |

## How to Run

```bash
docker-compose up --build
```

- **Front-end:** http://localhost:8080
- **API (Swagger):** http://localhost:5000/swagger

## Without Docker (dev)

```bash
# API
dotnet run --project src/CofreSenhas.Api

# Front-end
cd frontend
npm install
npm run dev
```

## Test Credentials

- **Email:** admin@cofre.com
- **Password:** admin123

## Features

- ✅ Login and registration with JWT
- ✅ Full CRUD for passwords
- ✅ Encrypted passwords (AES-256) in database
- ✅ Configurable password generator (length, uppercase, numbers, special chars)
- ✅ Strength indicator (Weak, Medium, Strong, Very Strong)
- ✅ One-click copy
- ✅ Show/hide password toggle
- ✅ Responsive dark mode interface

## Architecture

```
src/
├── CofreSenhas.Domain        ← Entities, DTOs, Interfaces, Enums
├── CofreSenhas.Service       ← Business logic (Auth, Passwords, Generator)
├── CofreSenhas.Persistence   ← EF Core + PostgreSQL, Repositories
└── CofreSenhas.Api           ← Controllers, JWT, Swagger
frontend/
└── React + TypeScript + Tailwind + Vite
tests/
└── CofreSenhas.Tests         ← xUnit
```

## Tests

```bash
dotnet test
```

## Author

Daniel Hoffmann
