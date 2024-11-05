FROM postgres:latest

# Copiar archivos de inicialización
COPY init/init.sql /docker-entrypoint-initdb.d/

# Variables de entorno
ENV POSTGRES_DB=midb
ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=postgres

# Exponer el puerto
EXPOSE 5434