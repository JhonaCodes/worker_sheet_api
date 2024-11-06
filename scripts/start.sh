#!/bin/bash

# Esperar a que la base de datos esté lista
echo "Waiting for database..."
while ! diesel setup 2>/dev/null; do
    echo "Database not ready. Waiting..."
    sleep 2
done

# Ejecutar migraciones
echo "Running database migrations..."
diesel migration run

# Iniciar la aplicación
echo "Starting application..."
exec worker_sheet_api