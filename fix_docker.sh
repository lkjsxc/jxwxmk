#!/bin/bash

echo "Fixing Docker issues..."

# Remove version from docker-compose.yml
sed -i '/^version:/d' docker-compose.yml

# Update Dockerfile to use newer Rust version
sed -i 's/FROM rust:1.70/FROM rust:1.80/' Dockerfile.backend

# Remove package-lock.json requirement from frontend Dockerfile
sed -i '/package-lock.json/d' Dockerfile.frontend

echo "Docker files updated successfully"
echo "Please start Docker daemon manually and run: docker compose up -d --build"