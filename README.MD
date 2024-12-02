# Worker Sheet API

![Rust](https://img.shields.io/badge/rust-1.78.0-orange.svg)
![Actix Web](https://img.shields.io/badge/actix--web-4.0-blue.svg)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-14+-blue.svg)
[![codecov](https://codecov.io/gh/jhonacodes/worker_sheet_api/graph/badge.svg?token=BNJWMICV1I)](https://codecov.io/gh/jhonacodes/worker_sheet_api)

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


# Docker y GitHub Packages 🐳

Este proyecto utiliza Docker para su despliegue y GitHub Packages como registro de contenedores. A continuación, se detallan los pasos para configurar y utilizar ambos servicios.

## Configuración de Docker 🛠️

### Requisitos Previos
- Docker instalado
- Docker Compose instalado
- Cuenta de GitHub
- Repositorio creado en GitHub

### Estructura de Archivos
```
.
├── Dockerfile
├── docker-compose.yml
├── scripts/
│   └── app.sh
└── .env (o dev.env para desarrollo)
```

### Ambientes Disponibles
El proyecto soporta dos ambientes:
- Desarrollo: `./scripts/app.sh dev`
- Producción: `./scripts/app.sh prod`

## Publicación en GitHub Packages 🚀

### 1. Configuración Inicial
1. En GitHub, navega a:
   - `Settings` → `Developer Settings` → `Personal Access Tokens` → `Tokens (Classic)`
2. Crea un nuevo token:
   - Nombre: `worker_sheet_api_key`
   - Permisos necesarios: `write:packages`, `delete:packages`
3. Guarda el token generado en `worker_sheet_api_key.txt` en tu proyecto

### 2. Autenticación
```bash
cat worker_sheet_api_key.txt | docker login docker.pkg.github.com -u tu_usuario --password-stdin
```

### 3. Publicación de la Imagen

#### Usando Docker Compose (Recomendado)

1. El `docker-compose.yml` debe incluir la configuración (usando minúsculas):
```yaml
services:
  api:
    build:
      context: .
      dockerfile: Dockerfile
    image: docker.pkg.github.com/jhonacodes/worker_sheet_api/worker_sheet_image
```

2. Construir y publicar:
```bash
# Construir y levantar contenedores
./scripts/app.sh prod

# Publicar en GitHub Packages
docker push docker.pkg.github.com/jhonacodes/worker_sheet_api/worker_sheet_image
```

### 4. Verificar Publicación
1. Ve a tu repositorio en GitHub
2. En el panel derecho, busca la sección "Packages"
3. O visita directamente:
   - `https://github.com/tu-usuario/worker_sheet_api/packages`
   - `https://github.com/users/tu-usuario/packages`

## Automatización con GitHub Actions 🤖

Para automatizar la publicación, crea el archivo `.github/workflows/docker-publish.yml`:

```yaml
name: Docker Build and Publish

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

env:
  REGISTRY: docker.pkg.github.com
  IMAGE_NAME: jhonacodes/worker_sheet_api/worker_sheet_image

jobs:
  build-and-push:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      packages: write

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Log in to GitHub Packages
        uses: docker/login-action@v3
        with:
          registry: ${{ env.REGISTRY }}
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Build and push Docker image
        uses: docker/build-push-action@v5
        with:
          context: .
          push: true
          tags: |
            ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:latest
            ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:${{ github.sha }}
```

## Control de Acceso 🔐

### Visibilidad
- Por defecto, las imágenes son privadas
- La visibilidad coincide con la del repositorio:
   - Repositorio privado → Imagen privada
   - Repositorio público → Imagen pública

### Acceso a Imágenes Privadas
Los usuarios necesitan:
1. Autenticación en GitHub Packages
2. Permisos en el repositorio
3. Token personal con permisos adecuados

### Gestión de Permisos
- Navega a: Repositorio → Settings → Collaborators and teams
- Añade usuarios o equipos
- Asigna permisos apropiados (read, write, admin)

## Uso de la Imagen 📦

### Pull de la Imagen
```bash
# Autenticación (necesaria para imágenes privadas)
docker login docker.pkg.github.com -u tu_usuario -p tu_token

# Descargar imagen
docker pull docker.pkg.github.com/jhonacodes/worker_sheet_api/worker_sheet_image
```

### Ejecución
```bash
docker run -d \
  --name worker_sheet_api \
  -p 8080:8080 \
  docker.pkg.github.com/jhonacodes/worker_sheet_api/worker_sheet_image
```

## Notas Importantes 📝

- Los nombres de repositorio e imagen DEBEN estar en minúsculas
- Si no se especifica versión, se usa `:latest` por defecto
- El token de acceso debe mantenerse seguro y no compartirse
- Se recomienda usar variables de entorno para configuración sensible
- Las imágenes son privadas por defecto para mayor seguridad
- Verifica la visibilidad deseada en la configuración del repositorio

## Solución de Problemas 🔧

### Errores Comunes
1. **Error de autenticación**:
   ```
   Error: authentication required
   ```
   Solución: Verifica tu token y vuelve a hacer login

2. **Error de nombre en mayúsculas**:
   ```
   invalid reference format
   ```
   Solución: Usa solo minúsculas en nombres de repositorio e imagen

3. **Error de permisos**:
   ```
   denied: permission denied
   ```
   Solución: Verifica los permisos del token y del repositorio

4. **Imagen no visible**:
   - Verifica que la imagen se subió correctamente con `docker push`
   - Revisa la sección "Packages" en tu repositorio
   - Confirma que tienes los permisos necesarios

### Contacto y Soporte
Para problemas adicionales:
- Abre un issue en el repositorio
- Contacta al equipo de desarrollo
- Consulta la [documentación oficial de GitHub Packages](https://docs.github.com/en/packages)


## 🔒 Licencia

Este proyecto está bajo una licencia propietaria. Ver el archivo [LICENCE](LICENCE) para más detalles.