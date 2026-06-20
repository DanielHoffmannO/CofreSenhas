🌐 [Português](README.md) | [Español](README.es.md)

# 🔐 Password Vault

Password vault with smart generator.

## Tech Stack

.NET 9 • EF Core • PostgreSQL • JWT • Vanilla JS • Docker

## How to Run

```bash
docker-compose up --build
```

- **API:** http://localhost:5000
- **Front-end:** http://localhost:8080
- **Swagger:** http://localhost:5000/swagger

## Test Credentials

- **Email:** admin@cofre.com
- **Password:** admin123

## Architecture

```
DDD: Domain → Service → Persistence → Api → Tests
```