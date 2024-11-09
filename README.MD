

<div align="center">
<h1>Worker Sheet API</h1> 

![Rust](https://img.shields.io/badge/rust-1.78.0-orange.svg)
![Actix Web](https://img.shields.io/badge/actix--web-4.0-blue.svg)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-14+-blue.svg)](https://www.postgresql.org)
![License](https://img.shields.io/badge/license-Proprietary-red.svg)
![Copyright](https://img.shields.io/badge/copyright-©%202024-red.svg)
![Rights](https://img.shields.io/badge/rights-All%20Reserved-red.svg)
![Usage](https://img.shields.io/badge/usage-Restricted-red.svg)
![Protection](https://img.shields.io/badge/protection-Maximum-darkred.svg)

Una API robusta para el sistema de bitácora Worker Sheet, construida con Rust, Actix-Web y Diesel.

[Documentación](#documentación) •
[Instalación](#instalación) •
[Uso](#uso) •
[Contribuir](#contribuir) •
[Licencia](#licencia)

</div>

## 🎯 Descripción

Worker Sheet API es un backend moderno y de alto rendimiento diseñado para servir como sistema de bitácora empresarial. Implementado en Rust, aprovecha la seguridad de tipos y el rendimiento del lenguaje junto con frameworks maduros como Actix-Web y Diesel ORM.

### Características Principales

- ✨ Arquitectura modular basada en features
- 🔐 Sistema de autenticación robusto
- 🚀 Alto rendimiento y baja latencia
- 🔄 API RESTful con documentación completa
- 🛡️ Seguridad y validación integradas

## 📚 Documentación

La documentación completa del proyecto está organizada en las siguientes secciones:

- [Actix Web Implementation](doc/actix.md)
   - Configuración y uso de Actix-Web
   - Estructuras de endpoints
   - Middleware y seguridad
   - Manejo de estados y respuestas

## 🚀 Instalación

### Prerequisitos

- Rust 1.78.0 o superior
- PostgreSQL 14+
- Diesel CLI

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Instalar Diesel CLI
cargo install diesel_cli --no-default-features --features postgres
```

### Configuración

1. Clonar el repositorio
```bash
git clone https://github.com/JhonaCodes/worker_sheet_api.git
cd worker_sheet_api
```

2. Configurar variables de entorno
```bash
cp .env.example .env
# Editar .env con tus configuraciones
```

3. Configurar la base de datos
```bash
diesel setup
diesel migration run
```

4. Compilar y ejecutar
```bash
cargo run
```

## 💻 Uso

### Endpoints Principales

```bash
# Autenticación
POST /api/auth/login
POST /api/auth/register

# Bitácoras
GET /api/sheets
POST /api/sheets
PUT /api/sheets/{id}
DELETE /api/sheets/{id}

# Usuarios
GET /api/users
POST /api/users
PUT /api/users/{id}
DELETE /api/users/{id}
```

Para más detalles, consulta la [documentación de la API](doc/actix.md).

## 🛠️ Desarrollo

### Estructura del Proyecto

```
src/
├── main.rs              # Punto de entrada
├── db.rs                # Configuración de base de datos
├── schema.rs            # Schema de Diesel        
├── auth/            # Autenticación
├── sheet/           # Lógica de bitácoras
└── user/            # Gestión de usuarios
```

### Comandos Útiles

```bash
# Ejecutar tests
cargo test

# Formatear código
cargo fmt

# Verificar código
cargo clippy

# Crear nueva migración
diesel migration generate nombre_migracion
```

## 📝 Licencia

Este proyecto está bajo `LICENCIA DE USO RESTRICTIVO Y CONFIDENCIALIDAD` - ver el archivo [LICENSE](LICENSE) para más detalles.

## 👥 Autores

- **Jhonatan Restrepo** - *Trabajo Inicial* - [JhonaCodes](https://github.com/JhonaCodes)

## 🙏 Agradecimientos

- Comunidad de Rust
- Contribuidores de Actix-Web
- Equipo de Diesel ORM

---
<div align="center">
Desarrollado con ❤️ por Jhonatan Ortiz
</div>