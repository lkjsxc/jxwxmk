# Source Tree Reconstruction

## 0) Prime directive (non-negotiable)

**Documentation is the source of truth.** Reconstruct the entire `src/` tree **from documentation only**.

* **Do not invent features** beyond documented behavior.
* If docs conflict: follow this priority order **exactly**:

  1. `docs/policy/INSTRUCT.md`
  2. `AGENTS.md`
  3. Remaining docs (closest scope wins: directory-local README > broader docs)
* When conflicts/ambiguities remain: make the **minimal** assumption that unblocks progress, then **record a decision** in docs (see Decision Log rule below) and continue.

---

## 1) Mandatory reads (before any action)

Before you write or change *any* code or docs, you must read and obey all of the following, in order:

### Always-required entrypoints

* `docs/policy/INSTRUCT.md`
* `AGENTS.md`
* `docs/README.md` (documentation index)
* `docs/technical/README.md` (architecture entrypoint)
* The closest `README.md` for every directory you will touch (read it *before* touching that directory)

### Scoped deep reads (follow TOCs; do not hard-code leaf files)

For each subsystem you will implement/touch, start at that subsystem’s directory `README.md` and **recursively read what its TOC links to** (children may be files or subdirectories).

Minimum expected coverage for full `src/` reconstruction:

* Backend: `docs/technical/backend/README.md`
* Frontend: `docs/technical/frontend/README.md`
* Config: `docs/technical/config/README.md`
* Deployment + runtime container: `docs/technical/deployment/README.md`
* Setup + Docker ergonomics: `docs/setup/README.md`
* Reconstruction plan + known gaps: `docs/plan/README.md` and `docs/implementation/README.md`

**If any mandatory file is missing/unreadable, STOP immediately** and request the missing file(s). Do not guess.

---

## 2) Hard constraints you must enforce continuously

These are invariants. Treat violations as build-breaking bugs.

### Repository structure constraints

* **Root allowlist only**: do not add new root-level items (directories/files).

  * If you need a new artifact, place it under an existing root path allowed by the repo (typically `docs/`, `src/`, etc.).
* **Exactly one `README.md` per directory** (no more, no less).

  * If a directory would otherwise need documentation, consolidate it into that single README.
* Prefer **small files and deep trees** (target **≤ 200 lines per source file**; split modules aggressively).
* If you create a directory, you must also create its one README in the same change.

### Runtime constraints

* Runtime is **one container** that runs **Rust server + PostgreSQL**.
* Node/TypeScript is **build-time only** (no long-running JS service in runtime).
* Server is authoritative; client is renderer + input only.

---

## 3) Architecture commitments (must match docs)

Implement these only insofar as docs require; do not extend behavior beyond docs.

* Fixed tick loop (target **20–60 Hz**) with a **single owner task** for world state.
* I/O handlers **enqueue events**; they **never mutate** world state directly.
* WebSocket is the gameplay channel; HTTP is only for **auth / health / static assets / admin**.
* Strict server-side validation: bounds, cooldowns, rate limits, schema validation.
* Persistence only for continuity (accounts, sessions, progression, village/world state); **no per-tick persistence**.
* Villages are first-class structures with: bounds, spawn link, and **≥ 1 interaction surface**.

---

## 4) Build + runtime requirements

* Multi-stage Docker build: **Node (esbuild)** → **Rust build** → **Debian runtime**.
* Rust server embeds compiled static assets from `src/static` via `rust-embed`.
* Runtime entrypoint starts PostgreSQL locally, then launches the Rust server.
* No separate Node process in runtime.

---

## 5) Protocol fidelity rules (zero drift)

* Implement all protocol messages **exactly** as documented.
* JSON protocol: explicit fields and enums; validate all inbound data strictly.
* If a field/message is unclear:

  * implement the **minimal** interpretation,
  * add a doc decision entry,
  * and make the implementation easy to revise later (isolate parsing/validation).

---

## 6) Testing requirements (treat as acceptance criteria)

* Unit tests for deterministic logic (crafting, combat, respawns, village rules) as required by docs.
* Integration tests for protocol handshake/auth and DB migrations.
* Tests must run in Docker or Docker Compose.
* “Done” requires tests passing in containerized environment.

---

## 7) Decision Log rule (required when unclear)

Whenever you encounter ambiguity, conflict, or undocumented but necessary detail:

* Add a short decision record in docs **without creating a new root item**.
* Prefer an existing decisions mechanism if docs specify one. If none exists, create under an already-allowed docs subtree (e.g., `docs/decisions/`) *only if that subtree is permitted by the repo’s existing root items and policy*.
* Each decision record must include:

  * Context + relevant doc references (file paths + section headings)
  * Options considered
  * Chosen minimal assumption
  * Impact on code/tree
  * Follow-up question if needed

---

## 8) Mandatory recursive workflow (Plan → Execute → Verify → Loop)

You must operate as a recursive agent: **every phase produces artifacts, runs checks, and either advances or loops back with corrections.**

### Phase A — Discovery & constraints extraction (no changes yet)

1. Read all mandatory docs.
2. Extract:

   * Required modules/components
   * Protocol messages and schemas
   * Persistence model + migrations requirements
   * Build pipeline requirements
   * Test requirements
   * Any explicit directory/file naming rules from docs/policy
3. Identify contradictions and ambiguities; log decisions (docs) only if needed.

**Gate A (must pass before Phase B):**

* You can enumerate all required subsystems from docs.
* You can state what is unknown/ambiguous (with doc pointers).
* No code changes have been made.

### Phase B — Full `src/` tree plan (docs-derived)

1. Derive a complete planned tree for `src/`:

   * directories, files, responsibilities
   * ownership boundaries (what mutates world state vs what enqueues events)
   * where protocol types live
   * where embedding assets happens
   * where config lives
   * where tests live
2. Enforce constraints in the plan:

   * one README per directory
   * small files (≤ 200 LoC target)
3. Produce/update **docs TOCs** to match the planned tree **before coding**.
4. Write/adjust directory `README.md` files to match planned responsibilities (one per directory).

**Gate B (must pass before Phase C):**

* Tree plan is complete and consistent with docs.
* Docs/TOCs reflect the plan.
* No implementation code exists that contradicts the plan.

### Phase C — Implement in thin vertical slices (recursive)

Implement using a loop of **small, testable slices**. Each slice must include:

1. Minimal implementation for a documented behavior
2. Strict validation + error handling
3. Tests required by docs for that behavior
4. Dockerized test run

**Slice order rule:** prefer foundational layers first:

* protocol types + validation → persistence/migrations → tick loop + event queue → gameplay systems → HTTP/WS I/O → frontend build/static assets → admin/health/auth as documented.

**Gate C (per slice):**

* The slice compiles.
* Relevant tests pass in Docker/Compose.
* No invented features; only doc-backed behavior.

### Phase D — Global verification & reconciliation (recursive)

After implementing slices:

1. Run full test suite in Docker/Compose.
2. Validate invariants:

   * no new root-level items
   * exactly one README per directory
   * protocol matches docs
   * tick loop ownership + event queue separation maintained
   * runtime single container: Rust + Postgres only
3. If any failure occurs (tests, build, invariants, doc drift), **loop back**:

   * Update plan/docs if the failure revealed doc mismatch
   * Adjust implementation to match docs
   * Add decision log only if docs are unclear
   * Re-run checks

**Stop condition (“Done”):**

* Full test suite passes in Docker/Compose
* Tree/doc structure matches plan
* All required behaviors implemented exactly as documented
* Decisions logged for every unresolved ambiguity

---

## 9) Operational rules while editing

* Make changes in **small commits** that preserve a working build as often as possible.
* Every directory touched must have its single README updated to remain accurate.
* Do not implement “nice-to-have” improvements unless docs require them.
* Prefer explicit types, explicit validation, and clear boundaries over cleverness.

---

## 10) Required output format (always use these headings, in order)

When responding, always output using **exactly** these headings in this order:

1. Intent
2. Plan
3. Repo tree impact
4. Docs impact
5. Code impact
6. Tests (Docker commands)
7. Commits (messages, in order)
8. Risks / Follow-ups

---

## 11) Final deliverables summary (mandatory at completion)

After completion, provide a concise summary of:

* Created directories/files (**top-level tree view only**)
* Assumptions or unresolved ambiguities (with doc references)
* Tests run and results (including Docker/Compose commands + outcomes)

---

## 12) Self-checklist (run mentally before final response)

* [ ] I read all mandatory docs first.
* [ ] I did not add new root-level items.
* [ ] Every directory touched has exactly one README.
* [ ] Planned tree was produced and docs/TOCs updated *before* coding.
* [ ] Implemented only doc-backed behavior.
* [ ] Protocol matches docs exactly; inbound validation is strict.
* [ ] Tick loop has single world owner; I/O only enqueues.
* [ ] Runtime is a single container: Rust server + Postgres; Node is build-time only.
* [ ] Tests run in Docker/Compose and pass.
* [ ] All ambiguities/conflicts are logged in docs.
