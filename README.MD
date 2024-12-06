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

# Configuración Automatizada de Nginx con Seguridad

## Descripción
Este repositorio contiene la documentación y scripts para configurar automáticamente Nginx con seguridad mejorada, incluyendo protección DDoS y Fail2ban.

## Contenido
1. Script de instalación y configuración
2. Documentación de la configuración
3. Guía de solución de problemas

## Script de Instalación
Guarda este script como `setup-nginx-security.sh`:

```bash
#!/bin/bash

# Variables
DOMAIN="workersheet.com"
EMAIL="info@calely.com"

# Función para mostrar mensajes
log_message() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1"
}

# Actualizar sistema
log_message "Actualizando sistema..."
sudo apt update
sudo apt upgrade -y

# Instalar dependencias
log_message "Instalando dependencias..."
sudo apt install -y nginx certbot python3-certbot-nginx fail2ban

# Backup de configuraciones existentes
log_message "Creando backups..."
sudo cp /etc/nginx/nginx.conf /etc/nginx/nginx.conf.backup
[ -f /etc/fail2ban/jail.local ] && sudo cp /etc/fail2ban/jail.local /etc/fail2ban/jail.local.backup

# Configurar Nginx para HTTP inicial
log_message "Configurando Nginx inicial..."
sudo cat > /etc/nginx/sites-available/default << EOL
server {
    listen 80;
    listen [::]:80;
    server_name ${DOMAIN} www.${DOMAIN};
    
    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_cache_bypass \$http_upgrade;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_read_timeout 90;
    }
}
EOL

# Configurar protección DDoS
log_message "Configurando protección DDoS..."
sudo cat > /etc/nginx/conf.d/ddos-protection.conf << 'EOL'
# Define zonas de limitación
limit_req_zone $binary_remote_addr zone=one:10m rate=30r/m;
limit_conn_zone $binary_remote_addr zone=addr:10m;

# Configuración de buffer y timeouts
client_max_body_size 10m;
client_body_buffer_size 128k;
client_header_buffer_size 1k;
large_client_header_buffers 4 4k;
client_body_timeout 60s;
client_header_timeout 60s;
send_timeout 60s;

# Configuración de FastCGI
fastcgi_read_timeout 600s;
fastcgi_send_timeout 600s;
fastcgi_connect_timeout 600s;
fastcgi_buffer_size 256k;
fastcgi_buffers 8 256k;
fastcgi_busy_buffers_size 256k;

# Configuración de seguridad
server_tokens off;
add_header X-XSS-Protection "1; mode=block";
add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline';" always;
add_header X-Frame-Options "SAMEORIGIN" always;
add_header X-Content-Type-Options "nosniff" always;
add_header Referrer-Policy "strict-origin-source" always;
add_header Permissions-Policy "geolocation=(), microphone=(), camera=()" always;

# Limitar conexiones y peticiones
limit_req zone=one burst=5 nodelay;
limit_conn addr 10;
EOL

# Configurar Fail2ban
log_message "Configurando Fail2ban..."
sudo cat > /etc/fail2ban/jail.local << 'EOL'
[DEFAULT]
bantime = 7200
findtime = 600
maxretry = 100
ignoreip = 127.0.0.1/8, ::1

[nginx-req-limit]
enabled = true
filter = nginx-req-limit
action = iptables-multiport[name=ReqLimit, port="http,https"]
logpath = /var/log/nginx/error.log
bantime = 7200
findtime = 600
maxretry = 100

[nginx-bad-bot]
enabled = true
filter = nginx-bad-bot
action = iptables-multiport[name=BadBot, port="http,https"]
logpath = /var/log/nginx/access.log
bantime = 7200
findtime = 600
maxretry = 50
EOL

# Crear filtros de Fail2ban
log_message "Creando filtros de Fail2ban..."
sudo cat > /etc/fail2ban/filter.d/nginx-req-limit.conf << 'EOL'
[Definition]
failregex = limiting requests, excess:.* by zone.*client: <HOST>
ignoreregex =
EOL

sudo cat > /etc/fail2ban/filter.d/nginx-bad-bot.conf << 'EOL'
[Definition]
failregex = ^<HOST> .*(GET|POST).*(\.php|\.asp|\.exe|\.pl|\.cgi|\.scgi).*$
            ^<HOST> .*(GET|POST).*(w00tw00t|phpMyAdmin|setup\.php|admin\.php).*$
ignoreregex =
EOL

# Reiniciar servicios
log_message "Reiniciando servicios..."
sudo systemctl stop nginx
sudo killall -9 nginx || true
sudo rm -f /var/run/nginx.pid
sudo systemctl start nginx
sudo systemctl restart fail2ban

# Obtener certificado SSL
log_message "Obteniendo certificado SSL..."
sudo certbot --nginx -d ${DOMAIN} -d www.${DOMAIN} --non-interactive --agree-tos -m ${EMAIL}

# Habilitar renovación automática
sudo systemctl enable certbot.timer
sudo systemctl start certbot.timer

# Verificación final
log_message "Verificando configuración..."
sudo nginx -t
sudo fail2ban-client status

log_message "Configuración completada. Tu sitio debería estar accesible en https://${DOMAIN}"
log_message "Comandos útiles:"
echo "fail2ban-client status                           # Ver estado general"
echo "fail2ban-client status nginx-req-limit          # Ver IPs baneadas por rate limit"
echo "fail2ban-client set nginx-req-limit unbanip IP  # Desbanear IP específica"
echo "fail2ban-client unban --all                     # Desbanear todas las IPs"
```

## Instrucciones de Uso

1. Crear el script:
```bash
nano setup-nginx-security.sh
```

2. Dar permisos de ejecución:
```bash
chmod +x setup-nginx-security.sh
```

3. Ejecutar el script:
```bash
./setup-nginx-security.sh
```

## Solución de Problemas

Si encuentras problemas durante la instalación:

1. Verificar que no hay servicios usando los puertos 80/443:
```bash
sudo netstat -tulpn | grep -E ':80|:443'
```

2. Verificar logs:
```bash
sudo tail -f /var/log/nginx/error.log
sudo tail -f /var/log/fail2ban.log
```

3. Verificar estado de servicios:
```bash
sudo systemctl status nginx
sudo systemctl status fail2ban
```

## Mantenimiento

### Comandos Útiles

#### Nginx:
```bash
sudo nginx -t                     # Verificar configuración
sudo systemctl restart nginx      # Reiniciar Nginx
sudo tail -f /var/log/nginx/*.log # Ver logs
```

#### Fail2ban:
```bash
sudo fail2ban-client status                  # Ver estado
sudo fail2ban-client status nginx-req-limit  # Ver detalles de jail
sudo fail2ban-client set nginx-req-limit unbanip IP  # Desbanear IP
```

## Notas de Seguridad
- Revisa regularmente los logs en `/var/log/nginx/` y `/var/log/fail2ban.log`
- Ajusta los valores de rate limiting según tus necesidades
- Mantén actualizada la lista de IPs permitidas en fail2ban
- Actualiza regularmente el sistema y los paquetes instalados

## Respaldo
Se recomienda hacer backups regulares de:
- `/etc/nginx/`
- `/etc/fail2ban/`
- Certificados SSL en `/etc/letsencrypt/`

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
