# AGENTS.md

## 0. Prime Directive

Build and evolve a **starve.io-like** multiplayer survival game as a **single integrated solution**:
- **Runtime processes**: **Rust game server** + **PostgreSQL** only.
- **TypeScript** is used for **build-time** generation of client assets (compiled to JS/CSS) that are **served by Rust**.
- No long-running Node/JS service in production.

This repository is developed **only by LLM agents**. Optimize for determinism, correctness, and fast iteration; prioritize **architectural coherence** over human-facing readability.

---

## 1. Hard Repo Invariants (Never Break)

### 1.1 Root layout
Project root contains **only**:
- `README.md`
- `LICENSE`
- `AGENTS.md`
- `docs/`
- `src/`
- hidden files/dirs (e.g. `.gitignore`, `.env.example`, `.github/`)

Anything else at root is a regression: move it under `docs/` or `src/`.

### 1.2 Documentation topology
- Every directory must contain **exactly one** `README.md` (acts as that directory’s table of contents).
- Additional documentation must live in:
  - subdirectories and/or
  - additional `.md` files
- Delete unused docs entirely (no “deprecated” placeholders).
- Each documentation file: **≤ 300 lines**.

### 1.3 Source code constraints
- Each source code file: **≤ 200 lines**.
- Prefer more files and deeper trees over long files.
- No backward compatibility requirements. Bold refactors are allowed when they simplify the long-term system.

### 1.4 Runtime constraints
- `docker compose up` must start:
  - one Rust service container
  - one Postgres container
- No extra runtime services (no Redis, no NATS, no separate web frontend).

---

## 2. Agent Operating Model

### 2.1 Default workflow (always)
1. **Locate constraints**: read the closest `README.md` in the relevant directories.
2. **Plan**: write a small internal checklist; identify affected modules and docs.
3. **Implement**: small cohesive changes; avoid “drive-by” unrelated edits.
4. **Test**: run fastest applicable checks (format, unit, minimal integration).
5. **Commit**: commit frequently (see 2.3).

### 2.2 Deterministic change discipline
- Prefer explicit state machines, fixed tick loops, bounded queues, and capped memory growth.
- Avoid cleverness. Prefer boring, auditable mechanisms.
- When fixing bugs, avoid ad-hoc patches; instead:
  - clarify invariants
  - improve types/structures
  - add tests that fail before the fix

### 2.3 Git commit policy (mandatory)
Commit **often**. Use this heuristic:
- If you changed behavior or added a feature slice: **commit**.
- If you changed schema/migrations: **commit** (separately).
- If you reorganized files/directories: **commit** (separately).
- If you updated docs to match code: **commit** (separately).

Commit message format:
- `feat(<area>): <summary>`
- `fix(<area>): <summary>`
- `refactor(<area>): <summary>`
- `docs(<area>): <summary>`
- `test(<area>): <summary>`
- `chore(<area>): <summary>`

Areas (examples): `net`, `world`, `combat`, `craft`, `db`, `assets`, `docker`, `protocol`, `ui`.

---

## 3. Architecture Commitments (Authoritative)

### 3.1 Server-authoritative multiplayer
- The server is the source of truth for:
  - positions/velocities
  - health/hunger/thirst/stamina
  - inventory/equipment
  - crafting outcomes
  - combat resolution
  - world resources and respawns
- Clients are “dumb renderers + input devices.”
- Never trust the client for anything that affects gameplay.

### 3.2 Networking model
- **WebSocket** for realtime gameplay.
- Optional HTTP endpoints (Actix Web) for:
  - login/session creation
  - static asset delivery
  - matchmaking/region selection (if needed)
- Use a **binary protocol** (preferred) or compact JSON with explicit versioning.
- Every message must have:
  - `protocol_version`
  - `msg_type`
  - `seq` (client input sequence)
- Include server tick in snapshots (`server_tick`) for reconciliation.

### 3.3 Simulation tick
- Fixed-rate tick loop (e.g., 20–60 Hz).
- Game loop runs as a single task owning world state:
  - processes input events from bounded channels
  - advances simulation deterministically
  - publishes snapshots/deltas to per-client outbound queues
- WebSocket handlers must not mutate world directly; they enqueue events.

### 3.4 Persistence boundaries (Postgres)
Persist:
- accounts / sessions
- player progression (cosmetics/unlocks if any)
- inventory/equipment (authoritative)
- long-lived world facts (optional; keep minimal early)

Do **not** persist:
- high-frequency transient state every tick
- per-tick physics minutiae

Prefer periodic checkpoint + event/delta persistence where needed.

### 3.5 Integrated “frontend” without a frontend service
- TypeScript lives in-repo to build browser client assets:
  - compile TS → JS (and bundle) during Docker build
  - Rust serves the compiled `/static/*`
- Node may exist only in **build stages**, never as a running service.
- Keep the client simple:
  - Canvas 2D
  - minimal sprites/shapes
  - emphasis on “rich content” via systems, not graphics complexity

---

## 4. Technology Stack Rules

### 4.1 Rust
- Async runtime: `tokio`
- Web framework: `actix-web` + WebSockets
- Prefer:
  - `tracing` for structured logs
  - `thiserror` for errors
  - `serde` for config/protocol where appropriate
- Avoid global mutable state; use ownership and channels.

### 4.2 Database
- PostgreSQL only.
- Migrations must be reproducible and committed.
- Schema changes require:
  - migration file
  - updated model layer
  - minimal integration test that exercises the new schema

### 4.3 TypeScript
- Client-only TS (browser).
- Keep runtime dependencies minimal; avoid heavy frameworks.
- Compile/bundle in Docker build (multi-stage), output to a Rust-served directory.

### 4.4 Docker Compose
- Compose is the canonical local run path.
- Runtime containers: Rust + Postgres only.
- Any “tools” containers must be dev-only and not required for `up` (avoid if possible).

---

## 5. Content Targets (Starve.io-like)

The game must prioritize **systemic depth** (“rich content”) with simple visuals.

### 5.1 Minimum gameplay pillars
- Gathering: wood/stone/food
- Crafting: tools, weapons, armor, buildings
- Survival meters: hunger (mandatory), optional thirst/temperature
- Combat: melee + at least one ranged option
- Progression: tech tiers via recipes/structures
- World: resource nodes with respawn rules; basic biomes
- Social: clans/teams or temporary alliances (optional early)

### 5.2 Simple graphics mandate
- Use:
  - primitive shapes, tiny sprites, or flat textures
  - consistent palette but not an asset-heavy pipeline
- Spend complexity budget on mechanics, not art.

---

## 6. Directory Strategy (Authoritative Pattern)

Agents should converge toward a deep, modular tree. Typical targets:

- `docs/`
  - `README.md` (docs index)
  - `architecture/`
  - `protocol/`
  - `gameplay/`
  - `operations/`
  - `decisions/` (ADRs)

- `src/`
  - `README.md` (code index)
  - `server/` (Rust)
  - `client/` (TypeScript source)
  - `assets/` (generated + static)
  - `db/` (migrations, seeds, schema docs)
  - `ops/` (docker, scripts, config templates)

Each directory must include its own `README.md` TOC.

---

## 7. Testing & Quality Gates

Minimum expectations per feature:
- Unit tests for deterministic logic (crafting, combat resolution, resource respawn).
- Integration tests for:
  - protocol handshake
  - DB migrations and basic persistence
- Load sanity:
  - simulate N clients with scripted inputs (even a crude harness is acceptable)

Always keep tests fast. Prefer a small number of high-value tests over exhaustive suites.

---

## 8. Security & Abuse Resistance (Baseline)

- Authentication:
  - session token (http-only cookie or bearer token)
- Rate limiting:
  - per-connection inbound message cap
  - disconnect on sustained abuse
- Validate all client inputs:
  - bounds checks
  - cooldown enforcement
  - server-side permission checks
- Never embed secrets in client assets.

---

## 9. Documentation Update Rules

Whenever code changes behavior or structure:
- Update the nearest directory `README.md` TOC.
- Add/adjust specific docs under `docs/` for:
  - protocol changes
  - major gameplay systems
  - operational changes (compose/env)

Delete docs that no longer match the system.

---

## 10. “Definition of Done” for a Slice

A slice is done when:
- Server sim supports it authoritatively.
- Client can render and interact with it.
- Protocol includes necessary messages with versioning.
- Persistence is implemented if required by design.
- Tests exist for critical logic.
- Docs/TOCs are updated.
- Changes are committed.

---

## 11. Non-Goals (For Now)

- Perfect anti-cheat (baseline validation only).
- Photorealistic art or heavy 3D.
- Multi-region infrastructure or autoscaling.
- Backward compatible protocol/schema evolution.

---

## 12. Agent Priority Order (When Tradeoffs Exist)

1. Correctness / determinism / server authority
2. Architectural simplicity under constraints (Rust+DB only)
3. Performance (avoid pathological patterns)
4. Content richness via reusable systems
5. Visual polish (last)
