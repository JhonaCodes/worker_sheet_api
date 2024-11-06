#!/bin/bash

# Colores para mejor visibilidad
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Función para detener contenedores existentes
stop_containers() {
    echo -e "${YELLOW}Deteniendo contenedores...${NC}"
    docker-compose down
}

# Función para producción usando docker-compose
run_prod() {
    echo -e "${GREEN}🚀 Iniciando ambiente de producción...${NC}"
    stop_containers
    echo -e "${GREEN}🚀 Iniciando aplicación con Docker Compose...${NC}"
    docker compose up --build
}

# Función para desarrollo local con DB en Docker
run_dev_local() {
    echo -e "${GREEN}🔧 Iniciando base de datos en Docker...${NC}"
    stop_containers
    docker-compose up db -d # -d para ejecutar en segundo plano

    echo -e "${YELLOW}⌛ Esperando que la base de datos esté lista...${NC}"
    # shellcheck disable=SC2046
    while ! docker exec $(docker-compose ps -q db) pg_isready 2>/dev/null; do
        echo -n "."
        sleep 1
    done
    echo -e "\n${GREEN}✅ Base de datos lista!${NC}"

    # Ejecutar migraciones primero
    echo -e "${YELLOW}🔄 Ejecutando migraciones...${NC}"
    # shellcheck disable=SC2046
    export $(cat dev.env | xargs) && diesel migration run
    echo -e "${GREEN}✅ Migraciones completadas${NC}"

    echo -e "${GREEN}🚀 Iniciando API localmente...${NC}"
    APP_ENV=dev cargo run
}

# Verificar argumento
case "$1" in
    "prod")
        run_prod
        ;;
    "dev")
        run_dev_local
        ;;
    *)
        echo "Uso: ./docker.sh [prod|local]"
        echo "  prod  - Inicia ambiente de producción con Docker Compose"
        echo "  dev - Inicia DB en Docker y API localmente (desarrollo)"
        exit 1
        ;;
esac