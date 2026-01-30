# Static Assets

Assets embedded in the Rust binary at compile time via rust-embed.

## Files

- `index.html`: Main HTML page
- `styles.css`: Base styles
- `game.js`: Built TypeScript bundle (do not edit directly)
- `favicon.ico`: Optional favicon

## Build Process

`src/client/index.ts` → (esbuild) → `src/static/game.js`

The Rust binary embeds this entire directory and serves it from memory.
