# Single Binary Deployment

To achieve a fully integrated frontend and backend with a minimal footprint, we utilize a single-binary architecture.

## Strategy
1.  **Frontend Build**: The TypeScript frontend is compiled to static assets (HTML, JS, CSS).
2.  **Asset Embedding**: These static assets are embedded directly into the Rust binary at compile time using `rust-embed`.
3.  **Server Hosting**: The Rust backend (`actix-web`) serves these embedded assets from memory.

## Benefits
-   **Zero Dependencies**: No need for Nginx, external static folders, or frontend servers.
-   **Portability**: The entire application is a single executable file.
-   **Performance**: Assets are served from memory (RAM), reducing I/O latency.

## Build Flow
1.  `npm run build` (Client) -> Generates `dist/` or `static/`.
2.  `cargo build --release` (Server) -> Embeds `static/` -> Produces binary.
