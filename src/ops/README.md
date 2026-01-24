# Operations Files

- `docker-compose.yml`: Compose setup.
- `Dockerfile`: Build instructions.

## Dockerfile
Multi-stage: Node for TS build, Rust for server.
Copies TS output to /static in Rust image.

## docker-compose.yml
Services: app (Rust), db (Postgres).
Ports: 8080 for app, 5432 for db.