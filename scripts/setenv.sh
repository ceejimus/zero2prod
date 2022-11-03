# source this file

export POSTGRES_USER=postgres
export POSTGRES_PASSWORD=password
export POSTGRES_PORT=5432
export POSTGRES_DB=newsletter

# for sqlx
export PGPASSWORD=${DB_PASSWORD}
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:${POSTGRES_PORT}/${POSTGRES_DB}
export DATABASE_URL