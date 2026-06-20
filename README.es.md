🌐 [Português](README.md) | [English](README.en.md)

# 🔐 Caja Fuerte de Contraseñas

Caja fuerte de contraseñas con generador inteligente.

## Tech Stack

.NET 9 • EF Core • PostgreSQL • JWT • Vanilla JS • Docker

## Cómo Ejecutar

```bash
docker-compose up --build
```

- **API:** http://localhost:5000
- **Front-end:** http://localhost:8080
- **Swagger:** http://localhost:5000/swagger

## Datos de Prueba

- **Email:** admin@cofre.com
- **Contraseña:** admin123

## Arquitectura

```
DDD: Domain → Service → Persistence → Api → Tests
```