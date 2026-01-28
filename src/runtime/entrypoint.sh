#!/bin/bash
set -e

echo "Starting PostgreSQL..."
service postgresql start

echo "Waiting for PostgreSQL..."
until su - postgres -c "pg_isready"; do
  sleep 1
done

echo "Configuring Database..."
su - postgres -c "psql -c \"ALTER USER postgres WITH PASSWORD 'postgres';\""
su - postgres -c "psql -c \"CREATE DATABASE jxwxmk;\"" || true

echo "Starting Server..."
exec ./server
