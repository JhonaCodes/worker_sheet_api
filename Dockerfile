# Usar imagen oficial de Rust
FROM rust:1.82.0 as builder

# Recibir los argumentos
ARG DATABASE_USER
ARG DATABASE_PASSWORD
ARG DATABASE_PORT
ARG DATABASE_NAME
ARG DATABASE_HOST
ARG DATABASE_URL
ARG HASH_SECRET
ARG JWT_SECRET
ARG API_KEY
ARG SMTP_EMAIL
ARG SMTP_EMAIL_PASSWORD
ARG SMTP_SERVER
ARG SMTP_SERVER_PORT

# Configurar las variables de entorno
ENV DATABASE_USER=$DATABASE_USER
ENV DATABASE_PASSWORD=$DATABASE_PASSWORD
ENV DATABASE_PORT=$DATABASE_PORT
ENV DATABASE_NAME=$DATABASE_NAME
ENV DATABASE_HOST=$DATABASE_HOST
ENV DATABASE_URL=$DATABASE_URL
ENV HASH_SECRET=$HASH_SECRET
ENV JWT_SECRET=$JWT_SECRET
ENV API_KEY=$API_KEY
ENV SMTP_EMAIL=$SMTP_EMAIL
ENV SMTP_EMAIL_PASSWORD=$SMTP_EMAIL_PASSWORD
ENV SMTP_SERVER=$SMTP_SERVER
ENV SMTP_SERVER_PORT=$SMTP_SERVER_PORT

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

# Crear y configurar el directorio de uploads
RUN mkdir -p /app/uploads && \
    chown -R nobody:nogroup /app/uploads && \
    chmod 755 /app/uploads

# Copiar la aplicación compilada y los scripts SQL desde la imagen builder
COPY --from=builder /app/target/release/worker_sheet_api /usr/local/bin/worker_sheet_api
COPY --from=builder /app/uploads /app/uploads
COPY sql /docker-entrypoint-initdb.d/
COPY scripts/start.sh /start.sh
RUN chmod +x /start.sh

# Exponer el puerto del servidor
EXPOSE ${SERVER_PORT}

# Iniciar la aplicación con el script start.sh
CMD ["/start.sh"]