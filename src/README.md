# Source Root

The source tree is divided into:

- `server/`: Rust backend (authoritative game server + database interactions).
- `client/`: TypeScript frontend (Canvas2D renderer + input handling).
- `static/`: Static assets (HTML, CSS, compiled JS) served by the backend.
- `runtime/`: Docker runtime configuration and entrypoints.
