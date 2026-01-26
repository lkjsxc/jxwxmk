# AGENTS.md — Authoritative Agent Policy and System Architecture

## 0. Prime Directive

Build and evolve a multiplayer survival game as a single integrated solution that is developed only by LLM agents.

**Runtime must be one container** that runs:
- the Rust game server (authoritative simulation + HTTP/WebSocket + static asset serving)
- PostgreSQL (inside the same runtime container)

TypeScript is allowed only for build-time generation of browser assets that are served by Rust. There is no long-running Node/JS service in production.

This document is authoritative for:
- repo invariants
- agent workflow and commit policy
- architecture commitments (server authority, tick loop, protocol)
- operational constraints (Docker-first, single runtime container)

`INSTRUCT.md` is the always-read execution contract; treat it as “how to act.”
This file is “what we are building” and “what must remain true.”

---

## 1. Repository invariants (hard constraints)

### 1.1 Root allowlist (strict)
Root contains only:
- `README.md`
- `LICENSE`
- `AGENTS.md`
- `docker-compose.yml`
- `.gitignore`
- `config/`
- `docs/`
- `src/`
- `.github/`
- hidden files/dirs

Anything else at root must be moved.

### 1.2 Documentation topology (recursive TOC)
- Every directory must contain exactly one `README.md` serving as that directory’s table of contents.
- Additional docs must live in subdirectories and/or additional `.md` files.
- Delete unused docs completely (no deprecated stubs).

### 1.3 Code shape
- Prefer deep trees and small files.
- Keep implementation boring and auditable: explicit types, invariants, and bounds.

### 1.4 Docker as the canonical execution environment
- Build and test in Docker.
- CI must run Docker builds/tests.
- Host-based workflows are convenience only; never required for correctness.

---

## 2. System architecture commitments (authoritative)

## 2.1 Server-authoritative simulation
The server is the single source of truth for:
- movement and physics resolution
- health and survival meters (hunger, etc.)
- inventory/equipment, crafting results, building placement
- combat resolution and damage application
- world resources, respawns, and structure state (including villages)

The client is a renderer + input device.
Never trust the client for anything that affects gameplay outcomes.

## 2.2 Fixed tick loop + single owner of world state
Simulation runs on a fixed-rate tick (target 20–60 Hz).
One task owns world state and is the only writer.

I/O handlers (WebSocket/HTTP) must never mutate world state directly. They enqueue events into bounded channels and receive snapshots/deltas via outbound queues.

This yields:
- deterministic-ish behavior
- clear ownership and fewer races
- bounded memory and backpressure points

## 2.3 Networking model
- WebSocket is primary for gameplay traffic.
- HTTP endpoints are allowed for:
  - login/session creation
  - health/readiness
  - serving static client assets
  - optional admin/dev endpoints (guarded)

Protocol requirements:
- explicit `protocol_version`
- explicit `msg_type`
- client input sequence `seq`
- server snapshots include `server_tick`

Protocol design principles:
- validate all inbound messages (bounds, cooldowns, permissions)
- disconnect for sustained abuse
- minimize ambiguity: explicit fields and enums over implicit meaning
- backwards compatibility is not required; we can bump versions aggressively

## 2.4 Persistence boundaries (PostgreSQL)
Persist only what is needed for continuity:
- accounts/sessions
- player progression and inventory/equipment
- village/world structure state as needed for long-lived world

Do not persist:
- per-tick transient physics minutiae
- high-frequency state every tick

Prefer:
- periodic checkpointing for stable state
- event logs for auditability (only where needed)

## 2.5 Integrated “frontend” without a frontend service
- Browser client assets are generated from TypeScript at build time.
- Rust serves compiled assets (e.g., `/static/*`).
- Node runs only in Docker build stages; it never runs as a server process.

Graphics mandate:
- Canvas2D, minimal primitives/sprites.
- Spend complexity on systems, not assets.

---

## 3. Security & abuse resistance (baseline)
- Authentication via session token (cookie or bearer).
- Per-connection rate limiting and message caps.
- Validate everything server-side:
  - bounds checks and type validation
  - cooldown enforcement
  - permissions and ownership checks
- Never embed secrets in client assets.

Parity rule:
Any “front-end restriction” must be matched by a “back-end restriction.”
Assume the client is adversarial.

---

## 4. Content pillars (what “the game” must contain)

### 4.1 Minimum pillars (must exist in some form)
- Gathering (wood/stone/food)
- Crafting (tools/weapons/armor, plus at least one structure)
- Survival meters: hunger mandatory; thirst/temperature optional but encouraged
- Combat: melee + at least one ranged option
- Progression: tech tiers via recipes/structures
- World: resource nodes with respawn rules; basic biome variation
- Social: at least party/clan scaffolding or proximity chat signals (optional early)

### 4.2 Village requirement (must be first-class)
Villages are world structures that shape:
- spawn/respawn
- safe zones or rule modifiers
- interaction surfaces (crafting, trade, quests, bulletin board, storage)
- points of interest and early-game onboarding

Minimum viable village:
- named region + bounds
- spawn point association
- at least one interactive station
- server-enforced ruleset differences (even if minimal: e.g., no PvP inside bounds)

---

## 5. Testing & quality gates

Minimum expectations per feature slice:
- Unit tests for deterministic logic (crafting, combat resolution, respawns, village rules).
- Integration tests for:
  - protocol handshake/auth
  - DB migrations and basic persistence path
- A basic load sanity harness is encouraged:
  - simulate N clients with scripted inputs
  - assert server remains stable and bounded

Tests must run in Docker.

---

## 6. Git policy (mandatory)

Commit frequently. Separate concerns:
- schema/migrations
- refactors/restructures
- behavior changes/features
- docs

Commit message format:
- `feat(<area>): <summary>`
- `fix(<area>): <summary>`
- `refactor(<area>): <summary>`
- `docs(<area>): <summary>`
- `test(<area>): <summary>`
- `chore(<area>): <summary>`

Areas examples: `net`, `protocol`, `world`, `village`, `combat`, `craft`, `db`, `assets`, `docker`, `ops`, `client`.

---

## 7 Tradeoff priority order (when forced)
1. Correctness and server authority
2. Determinism and bounded resource usage
3. Architectural simplicity under constraints (single runtime container)
4. Systemic content richness (reusable mechanics)
5. Visual polish (last)
