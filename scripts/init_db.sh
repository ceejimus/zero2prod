#!/usr/bin/env bash
set -x
set -eo pipefail

if [ -z "$(command -v psql)"]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if [ -z "$(command -v sqlx)"]; then
  echo >&2 "Error: sqlx ~0.6 is not installed."
  exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME=${POSTGRES_DB:=newsletter}
# Check if a custom port name has been set, otherwise default to '5432'
DB_PORT=${POSTGRES_PORT:=5432}

# Launch postgres using Docker
# "-N 1000" => increase maximum number of connections (for testing purposes)
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run -d \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    postgres postgres \
    -N 1000
fi

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD=${DB_PASSWORD}
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Waiting for postgres..."
  sleep 1
done

>&2 echo "Postgres available on port ${DB_PORT}!"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres DB is a GO!"