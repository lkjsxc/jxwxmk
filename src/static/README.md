# Static Assets

Static files embedded into the Rust server binary at compile time.

## Files

- `index.html` - Main HTML page
- `styles.css` - Base styling
- `game.js` - Built client bundle (output from `src/client/`)
- `favicon.ico` - Optional favicon

## Build

The client TypeScript builds to this directory:

```bash
cd src/client && npm run build
# Outputs: ../static/game.js
```

## Embedding

The Rust server uses `rust-embed` to include these files in the binary:

```rust
#[derive(RustEmbed)]
#[folder = "src/static/"]
struct StaticAssets;
```

This means the runtime container only needs the server binary—no separate static file serving required.
