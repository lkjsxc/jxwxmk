# Assets Crate

Static asset embedding and serving.

## Purpose

Embeds `src/static/` directory into the Rust binary at compile time using `rust-embed`.

## Functions

- `serve_index()` - Serve embedded `index.html`
- `serve_asset(filename)` - Serve any embedded file with MIME type detection
- `get_asset_list()` - List all embedded assets

## Build Flow

1. TypeScript client builds to `src/static/game.js`
2. Rust compile embeds `src/static/` via `RustEmbed` derive macro
3. Runtime binary serves assets from memory (no disk reads)
