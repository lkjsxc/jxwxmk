# Client Build Sources

TypeScript client source files (build-time only) that compile into `src/static/game.js`.

## Contents

- `package.json` / `tsconfig.json`: build setup.
- `index.ts`: runtime entrypoint.
- `index.html`, `styles.css`: unused runtime copies (kept for dev parity).
- `net/`: WebSocket client + protocol helpers.
- `input/`: input collection and cooldowns.
- `rendering/`: canvas loop, camera, visuals.
- `ui/`: canvas UI rendering.
- `state/`: client-side state cache.
- `utils/`: math and timing helpers.
