# En setup_database.sh
docker run --name postgres-db \
    --network=mi-red \
    -p 5434:5432 \
    -d mi-postgres

# En setup_api.sh
docker run -d \
    --name api-axum \
    --network=mi-red \
    -p 3000:3000 \
    api-axum