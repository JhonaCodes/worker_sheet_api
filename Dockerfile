# Dockerfile con multi-stage build para optimizar el tamaño y la seguridad
FROM rust:1.75-slim-bullseye as builder

# Instalar dependencias necesarias para la compilación
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Crear un directorio nuevo para nuestro código
WORKDIR /usr/src/app

# Copiar los archivos del proyecto
COPY . .

# Compilar la aplicación en modo release
RUN cargo build --release

# Usar la misma imagen base para la etapa final
FROM debian:bullseye-slim

# Instalar dependencias necesarias
RUN apt-get update && \
    apt-get install -y \
    libssl1.1 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copiar el binario compilado desde el builder
COPY --from=builder /usr/src/app/target/release/worker_sheet_api /usr/local/bin/worker_sheet_api

# Variables de entorno para la base de datos
ENV DATABASE_URL=postgres://postgres:postgres@postgres-db:5434/midb

# Puerto en el que correrá la aplicación
EXPOSE 3000

# Comando para ejecutar la aplicación
CMD ["/usr/local/bin/worker_sheet_api"]