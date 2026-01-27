# Frontend Build

The frontend is built at Docker build time and embedded into the Rust binary.

## Source Layout

- Entry: `src/client/index.ts`
- Runtime HTML/CSS: `src/static/index.html`, `src/static/styles.css`
- Output: `src/static/game.js` (bundled)

## Build Script

`src/client/package.json` defines:

- `npm run build`: bundles `index.ts` into `../static/game.js`.
- `npm run watch`: dev watch mode (not used in runtime).

## Notes

- `src/client/index.html` and `src/client/styles.css` exist but are not used in the runtime container.
- The server embeds `src/static/` and serves assets from memory.
