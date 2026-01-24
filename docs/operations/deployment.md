# Deployment and Build

## Docker Compose
- Services: Rust app, Postgres.
- Build: Multi-stage; compile TS in build stage, copy to Rust.

## Dockerfile
- Rust: Build binary.
- TS: Use Node for compilation, output to /static.

## Runtime
- `docker compose up`: Starts containers.
- No tools containers unless dev-only.