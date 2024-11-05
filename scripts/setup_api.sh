#!/bin/bash

# Obtener la ruta del directorio actual
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( dirname "$SCRIPT_DIR" )"

# Limpiar contenedor anterior si existe
docker stop api-axum 2>/dev/null || true
docker rm api-axum 2>/dev/null || true

# Crear red si no existe
docker network create mi-red 2>/dev/null || true

# Construir y ejecutar
docker build -t api-axum "$PROJECT_ROOT"
docker run -d \
    --name api-axum \
    --network=mi-red \
    -p 3000:3000 \
    api-axum

# Mostrar estado
echo "Estado del contenedor:"
docker ps | grep api-axum

echo "Logs del contenedor:"
docker logs api-axum