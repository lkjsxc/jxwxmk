# TypeScript Client Design

## Structure
- Build-time only: TS → JS/CSS during Docker build.
- `index.ts`: Entry; setup Canvas, WS connection.
- `game.ts`: Local simulation (predictive); reconcile with server ticks.
- `render.ts`: Draw primitives (shapes for entities, biomes).
- `input.ts`: Capture keyboard/mouse, send inputs to server.

## Rendering
- Canvas 2D: No heavy libs.
- Minimal assets: sprites/shapes; flat palette.
- Emphasis: Systems over graphics.

## Integration
- Compiled output to Rust-served `/static/*`.
- No runtime Node; browser-only JS.