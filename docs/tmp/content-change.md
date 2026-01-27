# Change Template Prompt (Expanded)

Use this prompt whenever you ask an LLM agent to make **any change** in this repo.

---

## 0) Prime directive
You are the only contributor. Follow repository policy exactly. Documentation is the source of truth.

If instructions conflict, **follow** `docs/policy/INSTRUCT.md` and `AGENTS.md`.

---

## 1) Mandatory reads (before any action)
- `docs/policy/INSTRUCT.md`
- `AGENTS.md`
- Closest `README.md` files for every directory you will touch (recursive TOCs).

If you cannot read a required file, stop and ask for it.

---

## 2) Scope guard (do this before planning)
- Restate the user request in one sentence.
- List **in-scope** items (bulleted).
- List **out-of-scope** items (bulleted).
- If the request implies new files or layout changes, ensure they **respect** root allowlist and README-per-directory rules.
- If the request conflicts with policy, **pause** and propose a compliant alternative.

---

## 3) Mandatory two-phase workflow
### Phase 1: Plan & structure
- Understand the repo and its constraints.
- Create a **detailed plan** (bullets with 3-7 steps).
- Update/add docs and TOCs **before** modifying source code.
- Keep docs and structure explicit; add or split files to keep docs within ~200-300 lines.

### Phase 2: Implement
- Modify code only after docs are updated.
- Keep files small (target <= 200 lines; split if needed).
- Add tests for deterministic logic and protocol behavior.
- Run tests in Docker (not on host).

---

## 4) Repository invariants (hard constraints)
- Root allowlist only (`README.md`, `LICENSE`, `AGENTS.md`, `.gitignore`, `config/`, `docs/`, `src/`, `.github/`, hidden files).
- Every directory must contain **exactly one** `README.md`.
- Prefer deep trees and small files.
- Single runtime container: Rust server + PostgreSQL inside the same container.
- TypeScript/Node allowed **only** at build-time to generate browser assets.

Never introduce a new root-level item. If the user asks for one, propose a compliant location.

---

## 5) Architecture commitments (summary)
- Server-authoritative simulation with a fixed tick loop.
- One owner task mutates world state; I/O only enqueues events.
- WebSocket for gameplay; HTTP only for health/login/static assets/admin.
- Validate all inbound messages; enforce rate limits; disconnect abuse.
- Client is rendering + input only; server decides outcomes.

---
config
## 6) Change hygiene
- Do not revert or overwrite unrelated changes in a dirty working tree.
- Prefer `rg` for search and `apply_patch` for small edits.
- If you must make assumptions, record them in docs (short rationale file).
- Keep edits minimal and aligned with the request; avoid scope creep.

---

## 7) Tests and validation
- Add unit tests for deterministic logic.
- Add integration tests for protocol handshake/auth and migrations when relevant.
- Run tests via Docker or Docker Compose.
- If tests are not run, explain why.

---

## 8) Required output format (PR-sized changes)
Use these headings **in order**:

1. Intent
2. Plan
3. Repo tree impact
4. Docs impact
5. Code impact
6. Tests (Docker commands)
7. Commits (messages, in order)
8. Risks / Follow-ups

Each section should be concise, bullet-oriented, and explicit.

---

## 9) Commit guidance
Commit frequently and separate concerns:
- `docs(<area>): ...`
- `feat(<area>): ...`
- `fix(<area>): ...`
- `refactor(<area>): ...`
- `test(<area>): ...`
- `chore(<area>): ...`

Areas: `net`, `protocol`, `world`, `village`, `combat`, `craft`, `db`, `assets`, `docker`, `ops`, `client`.

---

## 10) Deliverables checklist
- [ ] Read policy docs + relevant READMEs
- [ ] Declare scope (in/out)
- [ ] Plan with explicit steps
- [ ] Update docs + TOCs first
- [ ] Implement code changes
- [ ] Add tests
- [ ] Run tests in Docker
- [ ] Provide commit messages

---

### Task Input
(Insert the user request here.)
