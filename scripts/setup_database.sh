#!/bin/bash

# Colores para los mensajes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Configurando PostgreSQL con Docker...${NC}"

# Verificar si Docker está instalado
if ! command -v docker &> /dev/null; then
    echo -e "${RED}Docker no está instalado. Por favor, instala Docker primero.${NC}"
    exit 1
fi

# Detener y eliminar contenedor existente si existe
echo "Limpiando configuraciones anteriores..."
docker stop postgres-db &>/dev/null || true
docker rm postgres-db &>/dev/null || true

# Ubicación del script actual
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
PROJECT_ROOT="$( dirname "$SCRIPT_DIR" )"

# Construir y ejecutar el contenedor
echo -e "${BLUE}Iniciando contenedor de PostgreSQL...${NC}"
docker build -t mi-postgres "$PROJECT_ROOT/database" || {
    echo -e "${RED}Error al construir la imagen de Docker${NC}"
    exit 1
}

docker run --name postgres-db \
    -p 5434:5432 \
    -e POSTGRES_USER=postgres \
    -e POSTGRES_PASSWORD=postgres \
    -e POSTGRES_DB=midb \
    -d mi-postgres || {
    echo -e "${RED}Error al iniciar el contenedor${NC}"
    exit 1
}

echo -e "${GREEN}Esperando a que la base de datos esté lista...${NC}"
sleep 10  # Aumentado el tiempo de espera para asegurar que la BD esté lista

# Verificar que el contenedor está corriendo
if docker ps | grep -q postgres-db; then
    echo -e "${GREEN}Base de datos PostgreSQL está corriendo correctamente${NC}"
    echo -e "${BLUE}Puedes conectarte con:${NC}"
    echo "Host: localhost"
    echo "Puerto: 5434"
    echo "Usuario: postgres"
    echo "Base de datos: midb"

    # Verificar si el contenedor está realmente respondiendo
    if docker exec postgres-db pg_isready -U postgres > /dev/null 2>&1; then
        echo -e "${GREEN}La base de datos está respondiendo correctamente${NC}"
    else
        echo -e "${RED}La base de datos está iniciada pero no responde aún${NC}"
    fi
else
    echo -e "${RED}Hubo un problema al iniciar la base de datos${NC}"
    docker logs postgres-db
    exit 1
fi

# Mostrar los logs iniciales
echo -e "${BLUE}Últimos logs del contenedor:${NC}"
docker logs postgres-db --tail 10