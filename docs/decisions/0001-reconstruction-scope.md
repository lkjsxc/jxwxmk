# 0001 — Reconstruction scope

## Context

The documentation set mixes:

- **MVP-grade, implementable technical specs** (primarily under `docs/technical/`), and
- **long-term MMORPG targets** (primarily under `docs/design/`, especially `docs/design/mmorpg_elements.md`).

When an agent is asked to “reconstruct all of `src/` from docs”, the lack of an explicit scope boundary causes two common failure modes:

1. The agent attempts to implement every long-term MMORPG feature (parties, guilds, chat, moderation, etc.) and stalls.
2. The agent implements only a small core (HTTP + WS + some movement) and stops early, omitting documented systems.

This decision defines the scope for the **initial full reconstruction** so a single agent prompt can produce a complete, coherent implementation.

## Doc references

- `docs/policy/INSTRUCT.md` (invariants, Docker-first, single-container runtime)
- `docs/technical/README.md` (implementation architecture entrypoint)
- `docs/design/mmorpg_elements.md` (long-term target checklist)
- `docs/implementation/reconstruction_acceptance.md` (definition of “done”)

## Options considered

1. **Implement everything in `docs/design/` as “required now”**.
   - Pros: aligns with the “MMORPG Essentials” ambition.
   - Cons: too large and underspecified to be reliably completed in one reconstruction run.

2. **Implement only what is strictly specified in `docs/technical/`**.
   - Pros: implementable and testable.
   - Cons: risks missing critical player-facing loop items that exist only in design docs (e.g., some UX/interaction rules).

3. **Implement `docs/technical/` in full + a thin, explicitly bounded MVP slice of `docs/design/`**.
   - Pros: produces a playable, complete baseline while keeping scope bounded.
   - Cons: some long-term “essentials” remain planned.

## Decision (chosen)

Choose **Option 3**.

For the initial reconstruction, “complete” means:

- Implement all documented subsystems described under `docs/technical/` (backend, frontend, config, deployment), and
- Implement any design behaviors that are required to make those subsystems coherent and playable (world scale, settlements/villages, core loop, basic UX control rules),
- While treating explicitly marked **Optional Extensions / Future Ideas / Planned** sections as out-of-scope for the initial reconstruction.

The canonical completion checklist is: `docs/implementation/reconstruction_acceptance.md`.

## Planned / out of scope (for initial reconstruction)

These are intentionally **not required** for the initial reconstruction unless/ until they receive dedicated technical specs (protocol + persistence + UI):

- Parties, guilds/clans, multi-channel chat, friend/ignore lists (`docs/design/mechanics/social.md`, `docs/design/mmorpg_elements.md`)
- Moderation tooling, reporting, audit logs, and live-ops telemetry (`docs/design/mmorpg_elements.md`)
- Auction house / marketplace (`docs/design/mmorpg_elements.md`)
- Weather + seasonal systems (mentioned as optional extensions in various design docs)
- “100+ achievements” / “30–60 quest templates per tier” content targets (data volume targets; system should be data-driven, but only a small seed dataset is required)

## Impact

- Adds an explicit definition of “done” via `docs/implementation/reconstruction_acceptance.md`.
- Agents must use this scope when reconstructing `src/` so they neither stop early nor attempt the entire long-term MMO feature set.

## Follow-ups

- If/when parties/guilds/chat are promoted into scope, add:
  - protocol messages, persistence schema, and UI specs under `docs/technical/`
  - acceptance criteria updates in `docs/implementation/reconstruction_acceptance.md`
