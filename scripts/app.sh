#!/bin/bash

# Colores para mejor visibilidad
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Función para detener contenedores existentes
stop_containers() {
    echo -e "${YELLOW}🛑 Deteniendo contenedores...${NC}"

    # Verificar si hay contenedores ejecutándose
    if [ "$(docker compose ps -q)" ]; then
        docker compose --profile dev down
        echo -e "${GREEN}✅ Contenedores detenidos correctamente${NC}"
    else
        echo -e "${YELLOW}ℹ️  No hay contenedores ejecutándose${NC}"
    fi
}

# Función para producción usando docker-compose
run_prod() {
    echo -e "${GREEN}🚀 Iniciando ambiente de producción...${NC}"
    stop_containers
    echo -e "${GREEN}🚀 Iniciando aplicación con Docker Compose...${NC}"
    docker-compose --profile prod up --build -d
}

# Función para desarrollo local con DB en Docker y build completo
run_dev_local() {
    echo -e "${GREEN}🔧 Iniciando base de datos en Docker...${NC}"
    stop_containers

    # Cargar variables de entorno al inicio
    echo -e "${YELLOW}📝 Cargando variables de entorno desde dev.env${NC}"
    set -a
    source dev.env
    set +a

    export PGPASSWORD=${DATABASE_PASSWORD}

    echo -e "${YELLOW}📝 Variables cargadas:${NC}"
    echo -e "  Usuario: ${DATABASE_USER}"
    echo -e "  Base de datos: ${DATABASE_NAME}"
    echo -e "  Puerto: ${DATABASE_PORT}"
    echo -e "  Password: ${DATABASE_PASSWORD}"

    # Levantar el contenedor
    echo -e "${YELLOW}🚀 Iniciando contenedor de base de datos...${NC}"
    docker compose --env-file dev.env --profile dev up db_dev -d

    # Obtener el ID del contenedor
    CONTAINER_ID=$(docker compose ps -q db_dev)

    # Esperar a que el contenedor esté realmente running
    echo -e "${YELLOW}⌛ Esperando que el contenedor esté activo...${NC}"
    while ! docker inspect --format='{{.State.Running}}' $CONTAINER_ID 2>/dev/null | grep -q "true"; do
        echo -n "."
        sleep 1
    done
    echo -e "${GREEN}✅ Contenedor activo!${NC}"

    # Mostrar estado actual
    echo -e "${YELLOW}📊 Estado del contenedor:${NC}"
    docker compose ps

    # Mostrar logs iniciales
    echo -e "${YELLOW}📝 Logs del contenedor:${NC}"
    docker compose logs db_dev

    # Ahora intentar la conexión
    echo -e "${YELLOW}⌛ Verificando conexión a la base de datos...${NC}"
    MAX_RETRIES=30
    COUNTER=0

    while [ $COUNTER -lt $MAX_RETRIES ]; do
        if PGPASSWORD=${DATABASE_PASSWORD} psql -h localhost -p "${DATABASE_PORT}" -U "${DATABASE_USER}" -d "${DATABASE_NAME}" -c "SELECT 1" > /dev/null 2>&1; then
            echo -e "${GREEN}✅ Base de datos lista!${NC}"
            break
        fi
        if [ $((COUNTER % 5)) -eq 0 ]; then
            echo -e "\n${YELLOW}Estado actual del contenedor:${NC}"
            docker compose ps
            docker compose logs --tail=5 db_dev
        fi
        echo -n "-_"
        sleep 1
        COUNTER=$((COUNTER + 1))
    done

    if [ $COUNTER -eq $MAX_RETRIES ]; then
        echo -e "\n${RED}❌ Error: No se pudo conectar a la base de datos${NC}"
        echo -e "${YELLOW}📊 Logs completos:${NC}"
        docker compose logs db_dev
        exit 1
    fi

    # Resto del código igual...
    echo -e "${YELLOW}📦 Ejecutando scripts SQL...${NC}"
    for sql_file in sql/*.sql; do
        if [ -f "$sql_file" ]; then
            echo -e "${YELLOW}Ejecutando $sql_file...${NC}"
            PGPASSWORD=${DATABASE_PASSWORD} psql -h localhost -p ${DATABASE_PORT} -U "$DATABASE_USER" -d "$DATABASE_NAME" < "$sql_file"
            if [ $? -eq 0 ]; then
                echo -e "${GREEN}✅ Script $sql_file ejecutado correctamente${NC}"
            else
                echo -e "${RED}❌ Error ejecutando $sql_file${NC}"
            fi
        fi
    done

    echo -e "${YELLOW}📝 Información de conexión:${NC}"
    echo -e "  Host: localhost"
    echo -e "  Puerto: ${DATABASE_PORT}"
    echo -e "  Base de datos: ${DATABASE_NAME}"
    echo -e "  Usuario: ${DATABASE_USER}"
    echo -e "  Password: ${DATABASE_PASSWORD}"

    echo -e "${GREEN}🚀 Iniciando API localmente...${NC}"
    cargo run
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