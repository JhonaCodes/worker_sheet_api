# Usar imagen oficial de Rust
FROM rust:1.82.0 as builder

WORKDIR /app

# Instalar dependencias necesarias para la compilación
RUN apt-get update && \
    apt-get install -y \
    curl \
    libpq-dev \
    pkg-config \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Instalar rustup para manejar las versiones de Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Instalar clang y dependencias necesarias
RUN apt-get update && apt-get install -y \
    clang \
    llvm-dev \
    libclang-dev \
    libargon2-dev

# Copiar los archivos del proyecto
COPY sql /docker-entrypoint-initdb.d/
COPY src ./src
COPY Cargo.toml Cargo.lock ./

# Ejecutar la compilación
RUN cargo build --release

# Usar una imagen base más ligera para el contenedor final
FROM debian:bookworm-slim

# Instalar dependencias necesarias
RUN apt-get update && apt-get install -y \
    libssl-dev \
    libpq5 \
    ca-certificates \
    postgresql-client && \
    rm -rf /var/lib/apt/lists/*

# Copiar la aplicación compilada y los scripts SQL desde la imagen builder
COPY --from=builder /app/target/release/worker_sheet_api /usr/local/bin/worker_sheet_api
COPY sql /docker-entrypoint-initdb.d/
COPY scripts/start.sh /start.sh
RUN chmod +x /start.sh

# Exponer el puerto del servidor
EXPOSE ${SERVER_PORT}

# Iniciar la aplicación con el script start.sh
CMD ["/start.sh"]