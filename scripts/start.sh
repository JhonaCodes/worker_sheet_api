#!/bin/bash

# Esperar a que la base de datos esté lista
echo "Esperando a que la base de datos esté lista..."
until pg_isready -h "$DATABASE_HOST" -p "$DATABASE_PORT" -U "$DATABASE_USER"; do
  echo "Esperando a la base de datos..."
  sleep 1
done
echo "Base de datos está lista."

# Ejecutar scripts SQL en orden
echo "Ejecutando scripts de SQL..."
for sql_file in /docker-entrypoint-initdb.d/*.sql; do
  echo "Ejecutando $sql_file..."
  psql "$DATABASE_URL" -f "$sql_file"
done
echo "Scripts de SQL ejecutados."

# Iniciar la aplicación
echo "Iniciando la aplicación..."
exec /usr/local/bin/worker_sheet_api
