# Worker Sheet API

![Rust](https://img.shields.io/badge/rust-1.78.0-orange.svg)
![Actix Web](https://img.shields.io/badge/actix--web-4.0-blue.svg)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-14+-blue.svg)

API REST construida con Rust, Actix-Web y SQLx para gestión de bitácoras de trabajo.

## 📋 Prerequisitos

Antes de comenzar, asegúrate de tener instalado:

```bash
# 1. Rust y Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. PostgreSQL
sudo apt update
sudo apt install postgresql postgresql-contrib

# 3. Docker y Docker Compose
sudo apt install docker.io docker-compose
```

## 🚀 Inicio Rápido

### 1. Clonar e Instalar
```bash
# Clonar repositorio
git clone https://github.com/JhonaCodes/worker_sheet_api
cd worker_sheet_api

# Copiar archivos de ambiente
cp .env.example .env        # Para producción
cp .env.example dev.env     # Para desarrollo
```

### 2. Configurar Variables de Entorno

Edita los archivos `.env` y `dev.env` según tu entorno:

```env
# Valores ejemplo para dev.env (desarrollo)
APP_ENV=dev
DATABASE_USER=tu_usuario
DATABASE_PASSWORD=tu_password
DATABASE_PORT=5434         # Puerto diferente para desarrollo
DATABASE_NAME=worker_sheet_dev
DATABASE_HOST=localhost
SERVER_PORT=3000

# Para .env (producción) usa valores seguros y puerto 5432
```

### 3. Iniciando en Modo Desarrollo

Tienes dos opciones para iniciar el proyecto:

#### Opción 1: Usando el Script Automatizado
```bash
# Da permisos de ejecución
chmod +x scripts/app.sh

# Inicia en modo desarrollo
./scripts/app.sh dev
```

#### Opción 2: Manualmente
```bash
# 1. Inicia la base de datos en Docker
docker compose --env-file dev.env --profile dev up db_dev -d

# 2. Verifica que la base de datos esté activa
docker compose ps

# 3. Inicia la API localmente
cargo run
```

### 4. Iniciando en Modo Producción

#### Opción 1: Script Automatizado
```bash
./scripts/app.sh prod
```

#### Opción 2: Manualmente
```bash
# Construye e inicia todos los servicios
docker compose --profile prod up --build -d

# Verifica los servicios
docker compose ps
```

## 📊 Estructura del Proyecto

```
src/
├── auth/              # Autenticación y autorización
├── user/              # Gestión de usuarios
├── env/               # Configuración de entorno
└── static/            # Archivos estáticos
```

## 🔧 Comandos Útiles

### Docker
```bash
# Ver logs de la base de datos
docker compose logs db_dev     # Desarrollo
docker compose logs db        # Producción

# Detener servicios
docker compose --profile dev down  # Desarrollo
docker compose --profile prod down # Producción

# Reiniciar servicios
docker compose --profile dev restart  # Desarrollo
docker compose --profile prod restart # Producción
```

### Base de Datos
```bash
# Conectar a la base de datos (desarrollo)
psql -h localhost -p 5434 -U tu_usuario -d worker_sheet_dev

# Ver tablas
\dt

# Salir de psql
\q
```

### Rust
```bash
# Compilar y verificar errores
cargo check

# Ejecutar tests
cargo test

# Compilar para producción
cargo build --release
```

## 🌱 Ambientes

### Desarrollo
- Base de datos en contenedor Docker
- API ejecutándose localmente
- Hot-reloading disponible
- Puerto de base de datos: 5434

### Producción
- Base de datos y API en contenedores separados
- Optimizado para rendimiento
- Puerto de base de datos: 5432

## 📝 Notas Importantes

1. **Desarrollo vs Producción**
  - En desarrollo, solo la base de datos corre en Docker
  - En producción, tanto la API como la base de datos corren en containers

2. **Scripts SQL**
  - Los scripts en la carpeta `sql/` se ejecutan automáticamente al iniciar la base de datos
  - El orden de ejecución está definido en `init.sql`

3. **Variables de Entorno**
  - Nunca comitear `.env` o `dev.env` con contraseñas reales
  - Usar contraseñas seguras en producción

4. **Puertos**
  - API: 3000 (configurable en variables de entorno)
  - DB Desarrollo: 5434
  - DB Producción: 5432

## 🔍 Diagnóstico de Problemas

1. **La base de datos no inicia**
   ```bash
   # Verifica los logs
   docker compose logs db_dev
   
   # Verifica que el puerto no esté en uso
   sudo lsof -i :5434
   ```

2. **La API no compila**
   ```bash
   # Limpia los artefactos de compilación
   cargo clean
   
   # Actualiza las dependencias
   cargo update
   ```

3. **Error de conexión a la base de datos**
  - Verifica que las variables de entorno sean correctas
  - Confirma que el contenedor de la base de datos esté activo
  - Comprueba la conectividad al puerto correcto

## 🔒 Licencia

Este proyecto está bajo una licencia propietaria. Ver el archivo [LICENSE](LICENSE) para más detalles.