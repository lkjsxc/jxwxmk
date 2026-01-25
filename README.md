# Multiplayer Survival Game (Rust + Postgres)

Server-authoritative, deterministic(ish) multiplayer survival game inspired by *starve.io*.

This repository is built as a **single integrated solution**:
- **Runtime processes:** **Rust game server** + **PostgreSQL** only
- **Client:** browser assets generated at **build-time** from **TypeScript** and **served by Rust**
- **No long-running Node/JS service** in production (Node may be used only in Docker build stages)

Authoritative constraints and agent workflow rules live in **AGENTS.md**.

---

## What you get

- **Server-authoritative simulation**
  - Server owns positions/velocities, meters, inventory/equipment, crafting, combat, resources, respawns
  - Client is a renderer + input device; never trusted for game outcomes
- **Realtime networking**
  - WebSocket for gameplay
  - Optional HTTP endpoints for login/session + static asset delivery
- **Deterministic architecture bias**
  - Fixed tick loop, bounded queues, capped growth, explicit state machines
- **Minimal visuals, systemic depth**
  - Canvas 2D + simple primitives/sprites; complexity budget goes to mechanics

---

## Quickstart (local)

### Prerequisites
- Docker + Docker Compose v2

### Run
```bash
docker compose up --build
````

This starts exactly:

* 1 Rust server container
* 1 Postgres container

No additional runtime services should be introduced.

### Configure

If the repo includes a `.env.example`, copy it to `.env` and adjust values as needed:

```bash
cp .env.example .env
```

Operational details (ports, env vars, migrations behavior) should be documented under `docs/operations/`.

---

## How the system is structured

### Runtime topology

* **Rust server**: hosts HTTP + WebSocket, runs the simulation loop, serves compiled client assets
* **PostgreSQL**: accounts/sessions + long-lived progression/inventory (not per-tick transient state)

### Simulation model (high level)

* Fixed-rate tick loop (typically 20–60 Hz)
* The game loop is a single owner of world state:

  * consumes input events from bounded channels
  * advances simulation deterministically
  * publishes snapshots/deltas to per-client outbound queues
* WebSocket handlers enqueue events only; they do not mutate the world directly

### Protocol expectations

Every gameplay message must include:

* `protocol_version`
* `msg_type`
* `seq` (client input sequence)
  And snapshots must include:
* `server_tick` (for reconciliation)

See protocol documentation under `docs/protocol/`.

---

## Repository layout (root is strict)

Project root contains only:

* `README.md`
* `LICENSE`
* `AGENTS.md`
* `docs/`
* `src/`
* hidden files/dirs (e.g. `.gitignore`, `.env.example`, `.github/`)

If you add anything else at root, move it under `docs/` or `src/`.

---

## Where to read next

* `docs/README.md` — documentation index (architecture, protocol, gameplay, operations, decisions/ADRs)
* `src/README.md` — code index (server/client/assets/db/ops layout and entry points)

Each directory must contain **exactly one** `README.md` acting as its table of contents.

---

## Development workflow (LLM-agent repo)

This repo is developed **only by LLM agents**. The default workflow is:

1. Read the closest `README.md` in the directories you are changing.
2. Make small, cohesive changes (avoid unrelated edits).
3. Keep files short:

   * docs: ≤ 300 lines per `.md`
   * source: ≤ 200 lines per file
4. Add/adjust tests for deterministic logic and key integrations.
5. Commit frequently using the required format in **AGENTS.md**.

---

## Testing & quality gates (baseline)

Minimum expectations per feature slice:

* Unit tests for deterministic systems (crafting, combat resolution, respawns, etc.)
* Integration tests for:

  * protocol handshake
  * DB migrations + basic persistence paths
* A small load sanity harness is encouraged (scripted clients)

Run Rust checks (typical):

```bash
cargo fmt
cargo clippy
cargo test
```

Client build tooling is intentionally build-time only; see `src/client/README.md` for local iteration guidance.

---

## Security baseline

* Authentication via session token (cookie or bearer token)
* Rate limiting / abuse handling:

  * per-connection inbound caps
  * disconnect on sustained abuse
* Validate all client inputs (bounds, cooldowns, permissions)
* Never embed secrets in client assets

---

## License

See `LICENSE`.
