# 🔐 Cofre de Senhas

Cofre de senhas com gerador inteligente.

## Tech Stack

.NET 9 • EF Core • PostgreSQL • JWT • Vanilla JS • Docker

## Como Rodar

```bash
docker-compose up --build
```

- **API:** http://localhost:5000
- **Front-end:** http://localhost:8080
- **Swagger:** http://localhost:5000/swagger

## Dados de Teste

- **Email:** admin@cofre.com
- **Senha:** admin123

## Arquitetura

```
DDD: Domain → Service → Persistence → Api → Tests
```

## Autor

Daniel Hoffmann
