# Runtime Container

Docker build and runtime entrypoint for the single-container deployment (Rust server + PostgreSQL).

## Contents

- `Dockerfile`: Multi-stage build (Node -> Rust -> Debian runtime).
- `entrypoint.sh`: Starts PostgreSQL locally then launches the server binary.
