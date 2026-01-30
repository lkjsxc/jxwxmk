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

## Generated Data (from config)

Some UI surfaces need a server-aligned data table (e.g., crafting recipes) but the client cannot read `/app/config` at runtime.

Canonical approach:

- During the Node build stage, generate small TypeScript modules from repo config files and commit the generated modules into the bundle.
- Minimum required: generate the crafting recipe list from `config/crafting.json` for the crafting UI.

## Notes

- Runtime HTML/CSS live under `src/static/`. `src/client/` contains only TypeScript sources and build tooling.
- The server embeds `src/static/` and serves assets from memory.
