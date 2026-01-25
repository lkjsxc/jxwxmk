# Deployment

## Strategy
Docker Compose for all environments (Dev/Prod).

## Networking
- **Services**: `app` and `db` share the default bridge network `kkmypk_default`.
- **Database**:
    - Port `5432` is **NOT** exposed to the host for security.
    - The `app` container connects via the hostname `db`.

## Containers
- **App**: Rust Binary + Static Files.
- **DB**: PostgreSQL 16 (Alpine).