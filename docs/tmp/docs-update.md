# Docs-Only Update Prompt (Derived from content-change.md)

Use this prompt when the user explicitly wants **documentation-only** changes. This is a **constrained** variant of `prompt/content-change.md`.

**Rule:** Follow every instruction in `prompt/content-change.md` **unless** it conflicts with the docs-only constraints below. If it conflicts, the docs-only rule wins.

---

## 0) Docs-only constraints (override)
- **Do not modify source code, config, schemas, tests, or build scripts.**
- Update documentation files only (including `README.md` TOCs).
- If the user request **requires** code changes to be correct, stop and ask for clarification. Provide a docs-only alternative if possible.
- No scope creep: change only what the user asked for.

---

## 1) Mandatory reads (before any action)
- `docs/policy/INSTRUCT.md`
- `AGENTS.md`
- `prompt/content-change.md`
- Closest `README.md` files for every directory you will touch (recursive TOCs).

---

## 2) Scope guard (docs-only)
- Restate the user request in one sentence.
- List **in-scope** doc files/directories.
- List **out-of-scope** items (especially any code, config, or runtime changes).
- Confirm no root-level additions; if a new doc must be added, place it under an allowed directory and update TOCs.

---

## 3) Workflow (docs-only)
### Phase 1: Plan & structure
- Create a detailed plan (3-7 bullets).
- Update/add docs and TOCs first.
- Keep docs <= ~200-300 lines; split if needed.

### Phase 2: Implement (docs-only)
- Edit or add **only** `.md` files.
- Ensure each directory still has exactly one `README.md`.
- Remove stale or unused docs; no deprecated stubs.

---

## 4) Repository invariants (still apply)
- Root allowlist only; do not add new root items.
- Every directory must contain exactly one `README.md`.
- Prefer small docs and deep trees.

If a requested doc change would violate invariants, propose a compliant structure instead.

---

## 5) Output format (required)
Use these headings **in order**:
1. Intent
2. Plan
3. Repo tree impact
4. Docs impact
5. Code impact
6. Tests (Docker commands)
7. Commits (messages, in order)
8. Risks / Follow-ups

Docs-only notes:
- **Code impact** must explicitly state "None (docs-only)."
- **Tests** should say "Not run (docs-only)." unless the user asked for tests.

---

## 6) Commit guidance (docs-only)
Use `docs(<area>): <summary>` only.

---

## 7) Deliverables checklist
- [ ] Read policy docs + relevant READMEs
- [ ] Declare scope (docs-only)
- [ ] Plan with explicit steps
- [ ] Update docs + TOCs
- [ ] No code changes
- [ ] Provide commit messages

---

### Task Input
(Insert the user request here.)
