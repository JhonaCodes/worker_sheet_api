FROM rust:1.78.0 as builder

WORKDIR /app

# Instalar diesel_cli en el contenedor de construcción
RUN cargo install diesel_cli --no-default-features --features postgres

# Instalar dependencias necesarias para la compilación
RUN apt-get update && \
    apt-get install -y \
    libpq-dev \
    pkg-config

# Copiar archivos de migración
COPY migrations ./migrations
COPY src ./src
COPY Cargo.toml Cargo.lock ./
COPY diesel.toml ./

# Crear el archivo 'target' sin copiar el código fuente
RUN cargo build --release

# Usar una imagen base más reciente con GLIBC actualizado
FROM debian:bookworm-slim

# Instalar dependencias necesarias
RUN apt-get update && apt-get install -y \
    libssl-dev \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copiar la aplicación compilada y las migraciones
COPY --from=builder /app/target/release/worker_sheet_api /usr/local/bin/worker_sheet_api
COPY --from=builder /app/migrations /migrations
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Script de inicio para ejecutar migraciones y luego la aplicación
COPY scripts/start.sh /start.sh
RUN chmod +x /start.sh

EXPOSE ${SERVER_PORT}

CMD ["/start.sh"]