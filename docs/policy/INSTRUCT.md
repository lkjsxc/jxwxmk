# INSTRUCT.md — Always Read First

This repository is developed only by LLM agents. Optimize for deterministic iteration, architectural coherence, and correctness. Human-facing ergonomics are secondary; **agent-facing clarity and invariants are primary**.

## 0. Operating Contract (How you must work)

### 0.1 Mandatory two-phase execution
1) **Plan & structure first**
   - Clarify target behavior and invariants.
   - Update/reshape documentation and directory structure as needed.
2) **Implement second**
   - Write/modify source code following the plan.
   - Add tests and run them in Docker (not on host).

Never jump directly into code without first stabilizing the plan and the docs for the touched area.

### 0.2 Response format (every PR-sized change)
In your output, use these headings in this order:

1. **Intent**
2. **Plan**
3. **Repo tree impact**
4. **Docs impact**
5. **Code impact**
6. **Tests (Docker commands)**
7. **Commits (messages, in order)**
8. **Risks / Follow-ups**

Keep it crisp and explicit. Prefer checklists and bullet points over prose.

---

## 1. Hard repository invariants (do not violate)

### 1.1 Root layout (strict allowlist)
Project root may contain only:
- `README.md`
- `LICENSE`
- `AGENTS.md`
- `.gitignore`
- `config/`
- `docs/`
- `src/`
- `.github/`
- hidden files/dirs (e.g. `.env.example`, `.vscode/`)

Anything else at root is a regression. Move it under `docs/`, `src/`.

### 1.2 Documentation topology (recursive TOC discipline)
- **Every directory** must contain **exactly one** `README.md` (acts as that directory’s TOC and index).
- All other docs must live as:
  - additional `.md` files in the directory, and/or
  - subdirectories (each with their own `README.md`).
- Delete documentation that is no longer used. No “deprecated” placeholders.

### 1.3 Code topology (short files; deep trees)
- Prefer many small files and deep directories over large files.
- File size guidelines (soft, but strongly preferred for agent iteration):
  - source code: target ≤ 200 lines
  - docs: target ≤ 200–300 lines
  - if you must exceed, split the file instead of stretching the limit

### 1.4 Runtime constraint (single runtime container, includes database)
- The runtime environment must be **one container that runs**:
  - the **Rust game server**
  - **PostgreSQL** (inside the same container)
- There is **exactly one application** (the game server). Postgres is part of the runtime container, not a separate service.
- Node/TypeScript tooling is permitted **only at build-time** (Docker build stage), never as a long-running service.

### 1.5 Docker-first builds and tests
- Compilation and tests must run via Docker (and/or Docker Compose).
- Do not rely on host toolchains as the canonical path.

### 1.6 Security parity rule
As a general rule: **any restriction enforced on the frontend must also be enforced on the backend**.
- The server is authoritative for all gameplay outcomes and permissions.
- The client is never trusted for anything important.

---

## 2. Project identity (non-negotiable architecture direction)

### 2.1 Game type
A server-authoritative multiplayer survival game (inspired by starve.io-like simplicity) with:
- deterministic(ish) fixed tick simulation
- minimal visuals (Canvas2D, simple sprites/shapes)
- systemic depth (crafting, survival meters, building, combat, progression)

### 2.2 Village requirement (content anchor)
The world must include **villages** as first-class world structures.
At minimum, a village implies:
- a named area with boundaries and a spawn/respawn association
- at least one interaction surface (e.g., stash, crafting station, trader, bulletin board)
- safe-zone or ruleset differences (even if minimal)

“Add a village” is not optional—treat it as a baseline content primitive to shape world generation and progression.

---

## 3. Agent workflow rules (how changes are made)

### 3.1 Default workflow checklist
- [ ] Read closest directory `README.md` files for touched areas.
- [ ] Identify invariants (server authority, tick, persistence, security parity).
- [ ] Update docs / TOCs to reflect intended structure.
- [ ] Implement code in small cohesive slices.
- [ ] Add/adjust tests (unit + minimal integration).
- [ ] Run tests in Docker.
- [ ] Commit frequently with required format.

### 3.2 Determinism discipline
Prefer:
- explicit state machines
- fixed tick loop
- bounded queues / capped growth
- deterministic RNG (seeded) for simulation-affecting randomness

Avoid:
- unbounded memory growth
- ad-hoc patches
- client-side authority leakage
- “smart” but fragile implicit behavior

### 3.3 “Fixes must improve structure”
When fixing bugs:
- clarify invariants and failure modes
- improve types/structures to prevent recurrence
- add tests that fail before the fix

No duct tape.

### 3.4 Git discipline (mandatory)
Commit frequently:
- behavior changes and feature slices
- schema/migrations
- refactors/restructures
- docs-only updates

Commit message format:
- `feat(<area>): <summary>`
- `fix(<area>): <summary>`
- `refactor(<area>): <summary>`
- `docs(<area>): <summary>`
- `test(<area>): <summary>`
- `chore(<area>): <summary>`

Areas examples: `world`, `net`, `protocol`, `db`, `assets`, `docker`, `ops`, `client`, `combat`, `craft`, `village`.

---

## 4. Quality bar (definition of done for any slice)
A slice is done only when:
- server implements it authoritatively (tick-owned state)
- client can exercise it (even minimally)
- protocol supports it explicitly (versioned; validated)
- persistence exists if required (otherwise documented why not)
- tests exist for critical logic
- docs and TOCs are updated
- commits are clean and ordered

If you cannot reach full DoD, explicitly list what is missing and why.

---

## 5. If you are uncertain
Do not stall. Make the best coherent decision consistent with this document and:
- write the decision into docs (ADR or a short rationale file)
- implement accordingly
- leave a follow-up note

This repo favors forward motion with explicit decisions over ambiguity.
