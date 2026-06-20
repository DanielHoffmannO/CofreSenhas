🌐 [Português](README.md) | [English](README.en.md)

# 🔐 Caja Fuerte de Contraseñas

Caja fuerte de contraseñas con generador inteligente, indicador de fuerza y cifrado AES-256.

## Tech Stack

| Capa | Tecnología |
|------|-----------|
| Front-end | React 18 + TypeScript + Tailwind CSS + Vite |
| Back-end | .NET 9 / ASP.NET Core Web API |
| Base de datos | PostgreSQL 16 |
| Auth | JWT (Bearer Token) |
| Cifrado | AES-256 (contraseñas almacenadas cifradas) |
| Infra | Docker Compose |

## Cómo Ejecutar

```bash
docker-compose up --build
```

- **Front-end:** http://localhost:8080
- **API (Swagger):** http://localhost:5000/swagger

## Sin Docker (dev)

```bash
# API
dotnet run --project src/CofreSenhas.Api

# Front-end
cd frontend
npm install
npm run dev
```

## Datos de Prueba

- **Email:** admin@cofre.com
- **Contraseña:** admin123

## Características

- ✅ Login y registro con JWT
- ✅ CRUD completo de contraseñas
- ✅ Contraseñas cifradas (AES-256) en la base de datos
- ✅ Generador de contraseñas configurable (tamaño, mayúsculas, números, especiales)
- ✅ Indicador de fuerza (Débil, Media, Fuerte, Muy Fuerte)
- ✅ Copiar con 1 clic
- ✅ Mostrar/ocultar contraseña
- ✅ Interfaz dark mode responsiva

## Arquitectura

```
src/
├── CofreSenhas.Domain        ← Entidades, DTOs, Interfaces, Enums
├── CofreSenhas.Service       ← Reglas de negocio (Auth, Contraseñas, Generador)
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

## Autor

Daniel Hoffmann
