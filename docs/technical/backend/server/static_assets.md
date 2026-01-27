# Static Assets

The Rust server embeds frontend assets at compile time and serves them from memory.

## Build Flow

1. `npm run build` in `src/client/` bundles `index.ts` into `src/static/game.js`.
2. `cargo build --release` compiles the server and embeds `static/` using `rust-embed`.
3. The runtime container only needs the Rust binary; assets are baked in.

## Serving Logic

- `serve_index` returns embedded `index.html`.
- `serve_asset` returns any embedded file under `static/` and infers the MIME type using `mime_guess`.

## Static Directory Layout

`src/static/` should contain:

- `index.html`
- `styles.css`
- `game.js` (built output)
- `favicon.ico`
