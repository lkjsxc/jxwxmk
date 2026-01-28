# 01 — Foundation (repo + `src/` skeleton)

Goal: establish the minimal repo structure so all later slices have a stable place to land.

References:
- `docs/policy/INSTRUCT.md` (root allowlist, one README per directory, Docker-first)
- `docs/technical/module_map.md` (module boundaries)
- `docs/technical/tech_stack.md` (libraries)

## A) Root invariants (must hold before any implementation)

- [ ] Root contains only allowlisted items from `docs/policy/INSTRUCT.md` (no new root files/dirs).
- [ ] Every directory created under `src/` includes **exactly one** `README.md`.
- [ ] Every directory created under `config/` includes **exactly one** `README.md` (if subdirs are introduced later).

## B) Create the `src/` skeleton (no behavior yet)

Recommended module layout (aligns with `docs/technical/module_map.md`):

- `src/README.md` (source tree map; links to key sub-READMEs)
- `src/runtime/` (Docker runtime wiring)
- `src/static/` (embedded assets served by Rust)
- `src/client/` (TypeScript client; build-time only)
- `src/server/` (Rust crate: server + game + persistence)

Tasks:

- [ ] Create `src/README.md` describing the intended tree and entrypoints.
- [ ] Create `src/runtime/README.md` explaining Docker/entrypoint responsibilities.
- [ ] Create `src/runtime/compose/README.md` explaining compose file variants (files live in `src/runtime/compose/`).
- [ ] Create `src/static/README.md` describing required files (`index.html`, `styles.css`, `game.js`, optional `favicon.ico`).
- [ ] Create `src/client/README.md` describing build outputs and constraints (no runtime Node).
- [ ] Create `src/server/README.md` describing crates/modules and the single-writer tick model.

## C) Rust crate scaffolding (inside `src/`)

References:
- `docs/technical/backend/server/overview.md` (Actix, workers(1))
- `docs/technical/backend/server/static_assets.md` (`rust-embed`)
- `docs/technical/backend/database/README.md` (`sqlx`, Postgres URL)

- [ ] Create a Rust workspace under `src/server/` (keep root allowlist: do not add `Cargo.toml` at repo root).
- [ ] Split boundary modules into separate crates to enforce `docs/technical/module_map.md` at compile time:
  - `protocol`, `config`, `world`, `systems`, `game`, `persistence`, `net`, `assets`
- [ ] Create a small binary crate that wires adapters + engine together (avoid a “god module”).
- [ ] Add dependencies per `docs/technical/tech_stack.md`:
  - `actix-web`, `actix`, `actix-web-actors`
  - `serde`, `serde_json`
  - `uuid`, `rand`
  - `rust-embed`, `mime_guess`
  - `log`, `env_logger`
  - `sqlx` (Postgres)
- [ ] Add a minimal binary entrypoint that starts Actix and exposes `GET /health` and `GET /metrics` (full behavior later).

## D) TypeScript client scaffolding (inside `src/`)

References:
- `docs/technical/frontend/build.md`
- `docs/technical/frontend/runtime.md`

- [ ] Create `src/client/package.json` with `esbuild` build script outputting `../static/game.js`.
- [ ] Create `src/client/tsconfig.json` (ES2020 target).
- [ ] Create `src/client/index.ts` skeleton that can be bundled (full runtime later).

## E) Static assets scaffolding

References:
- `docs/technical/backend/server/static_assets.md`
- `docs/technical/frontend/build.md`

- [ ] Create `src/static/index.html` that loads `styles.css` and `game.js`.
- [ ] Create `src/static/styles.css` (minimal base styling).
- [ ] Ensure the server will embed `src/static/` via `rust-embed` (implementation later).

## Done when

- [ ] All directories above exist with one `README.md` each.
- [ ] The repo can build a “skeleton” Docker image (even if gameplay is not implemented yet).
