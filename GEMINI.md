# AGENTS.md — Authoritative Agent Policy and System Architecture

This repository is developed only by LLM agents. Documentation is the source of truth.

## 0. Prime Directive

Build and evolve a multiplayer survival game as a single integrated solution.

Runtime must be one container that runs:
- the Rust game server (authoritative simulation + HTTP/WebSocket + static asset serving)
- PostgreSQL (inside the same runtime container)

TypeScript is allowed only for build-time generation of browser assets that are served by Rust. There is no long-running Node/JS service in production.

`docs/policy/INSTRUCT.md` is the always-read execution contract. Treat it as the how-to-work policy. This file is what we are building and what must remain true.

---

## 1. Repository Invariants (Hard Constraints)

### 1.1 Root Allowlist (Strict)
Root contains only:
- README.md
- LICENSE
- AGENTS.md
- .gitignore
- config/
- docs/
- src/
- .github/
- hidden files/dirs

Anything else at root must be moved under docs/ or src/.

### 1.2 Documentation Topology (Recursive TOC)
- Every directory must contain exactly one README.md (the directory TOC).
- Additional docs must live in that directory or subdirectories (each with their own README.md).
- Delete unused docs; no deprecated stubs.

### 1.3 Code Shape
- Prefer small files and deep trees.
- Keep implementation boring and auditable: explicit types, invariants, bounds.

### 1.4 Docker-First
- Build and test in Docker.
- CI must run Docker builds/tests.
- Host workflows are convenience only; never required for correctness.

---

## 2. System Architecture Commitments

### 2.1 Server-Authoritative Simulation
The server is the single source of truth for:
- movement and physics
- survival meters (hunger, etc.)
- inventory/equipment, crafting, building placement
- combat resolution and damage
- world resources, respawns, and structures (including villages)

The client is a renderer + input device. Never trust the client for gameplay outcomes.

### 2.2 Fixed Tick Loop + Single Owner
- Simulation runs at a fixed tick rate (target 20-60 Hz).
- One task owns world state and is the only writer.
- I/O handlers must enqueue events; they must not mutate world state directly.

### 2.3 Networking Model
- WebSocket is primary for gameplay traffic.
- HTTP is allowed for:
  - login/session creation
  - health/readiness
  - serving static assets
  - optional admin/dev endpoints (guarded)

Protocol principles:
- validate all inbound messages (bounds, cooldowns, permissions)
- disconnect for sustained abuse
- explicit fields and enums; avoid implicit meaning
- backwards compatibility is not required (version bump freely)

### 2.4 Persistence Boundaries
Persist only what is needed for continuity:
- accounts/sessions
- player progression and inventory
- village/world structure state as needed

Do not persist:
- per-tick transient physics
- high-frequency state every tick

Prefer periodic checkpoints for stable state.

### 2.5 Integrated Frontend
- Browser client assets are generated from TypeScript at build time.
- Rust serves compiled assets from embedded static files.
- Node is build-time only.

Graphics mandate:
- Canvas2D, minimal primitives/sprites.
- Spend complexity on systems, not assets.

---

## 3. Security and Abuse Resistance

- Authentication via session token (cookie or bearer).
- Per-connection rate limiting and message caps.
- Validate everything server-side.
- Never embed secrets in client assets.

Parity rule: any frontend restriction must be matched by a backend restriction.

---

## 4. Content Pillars (Must Exist)

### 4.1 Minimum Pillars
- Gathering (wood/stone/food)
- Crafting (tools/weapons/armor, plus at least one structure)
- Survival meters: hunger mandatory; thirst/temperature optional
- Combat: melee + at least one ranged option
- Progression: tech tiers via recipes/structures
- World: resource nodes with respawn rules; basic biome variation
- Social: party/clan scaffolding or proximity chat (optional early)

### 4.2 Village Requirement
Villages are first-class structures that shape:
- spawn/respawn
- safe zones or rule modifiers
- interaction surfaces (crafting, trade, quests, storage)
- points of interest and onboarding

Minimum viable village:
- named region + bounds
- spawn association
- at least one interactive station
- server-enforced ruleset difference (even if minimal)

---

## 5. Testing and Quality Gates

Minimum expectations per feature slice:
- Unit tests for deterministic logic (crafting, combat, respawns, village rules).
- Integration tests for protocol handshake/auth and DB migrations.
- Optional load sanity harness (scripted clients, bounded memory).

Tests must run in Docker.

---

## 6. Git Policy

Commit frequently. Separate concerns:
- schema/migrations
- refactors/restructures
- behavior changes/features
- docs

Commit message format:
- feat(<area>): <summary>
- fix(<area>): <summary>
- refactor(<area>): <summary>
- docs(<area>): <summary>
- test(<area>): <summary>
- chore(<area>): <summary>

Areas: net, protocol, world, village, combat, craft, db, assets, docker, ops, client.

---

## 7. Tradeoff Priority Order

1. Correctness and server authority
2. Determinism and bounded resource usage
3. Architectural simplicity under single-container constraint
4. Systemic content richness
5. Visual polish
