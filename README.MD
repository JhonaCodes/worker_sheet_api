# Sistema de Gestión de Trabajadores

Este proyecto implementa un sistema de gestión de trabajadores utilizando una arquitectura de microservicios con Docker. Consiste en una API REST desarrollada en Rust con Axum y una base de datos PostgreSQL.

## Requisitos Previos

- Docker v20.10.0 o superior
- Docker Compose v2.0.0 o superior
- Git

## Estructura del Proyecto

```
.
├── api/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── Dockerfile
├── database/
│   ├── init/
│   │   └── init.sql
│   └── Dockerfile
├── scripts/
│   ├── setup_api.sh
│   └── setup_database.sh
└── docker-compose.yml
```

## Configuración Inicial

1. Clonar el repositorio:
```bash
git clone <url-del-repositorio>
cd sistema-trabajadores
```

2. Configurar variables de entorno (opcional):
```bash
# Por defecto se usarán estas configuraciones
export POSTGRES_USER=postgres
export POSTGRES_PASSWORD=postgres
export POSTGRES_DB=midb
export API_PORT=3000
```

## Instalación y Ejecución

### Método 1: Usando Docker Compose (Recomendado)

1. Iniciar todos los servicios:
```bash
docker-compose up -d
```

2. Verificar que los servicios estén corriendo:
```bash
docker-compose ps
```

### Método 2: Usando Scripts Individuales

1. Crear la red de Docker:
```bash
docker network create mi-red
```

2. Iniciar la base de datos:
```bash
chmod +x scripts/setup_database.sh
./scripts/setup_database.sh
```

3. Iniciar la API:
```bash
chmod +x scripts/setup_api.sh
./scripts/setup_api.sh
```

## Verificación del Sistema

1. Comprobar el estado de la API:
```bash
curl http://localhost:3000/
# Debería responder: "API Funcionando!"
```

2. Listar trabajadores:
```bash
curl http://localhost:3000/workers
```

3. Obtener un trabajador específico:
```bash
curl http://localhost:3000/workers/1
```

## Detener el Sistema

### Con Docker Compose:
```bash
docker-compose down
```

### Con Scripts Individuales:
```bash
docker stop api-axum postgres-db
docker rm api-axum postgres-db
```

## Estructura de la Base de Datos

La tabla principal `workers` tiene la siguiente estructura:

```sql
CREATE TABLE workers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    position VARCHAR(100) NOT NULL,
    department VARCHAR(100),
    salary DECIMAL(10,2),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## Endpoints de la API

| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | / | Health check |
| GET | /workers | Lista todos los trabajadores |
| GET | /workers/:id | Obtiene un trabajador específico |

## Solución de Problemas Comunes

1. **Error de conexión a la base de datos**:
    - Verificar que la red de Docker esté creada: `docker network ls`
    - Comprobar los logs de la base de datos: `docker logs postgres-db`

2. **API no responde**:
    - Verificar los logs de la API: `docker logs api-axum`
    - Comprobar que los puertos estén correctamente expuestos: `docker ps`

3. **Problemas de permisos en los scripts**:
    - Asegurar que los scripts tienen permisos de ejecución: `chmod +x scripts/*.sh`

## Desarrollo Local

Para desarrollar localmente, puedes modificar la URL de la base de datos en las variables de entorno:

```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/midb
```

## Contribución

1. Fork el repositorio
2. Crea una rama para tu feature: `git checkout -b feature/nueva-caracteristica`
3. Commit tus cambios: `git commit -am 'Agregar nueva característica'`
4. Push a la rama: `git push origin feature/nueva-caracteristica`
5. Crea un Pull Request

## Contacto

- Desarrollador: Jhonatan Ortiz
- Universidad: Politécnico Gran Colombiano
- Año: 2024

## Licencia

Este proyecto está bajo la Licencia MIT. Ver el archivo `LICENSE` para más detalles.