# Reconstruction Scope (Initial)

This document defines the scope boundary for the **initial full reconstruction** of `src/` from the documentation in `docs/`.

It exists to prevent two common failure modes:

1. An agent attempts to implement every long-term “MMO wishlist” item and stalls.
2. An agent implements only a tiny core (HTTP + WS + movement) and stops early, omitting documented systems.

The canonical “done” checklist remains: `docs/implementation/reconstruction_acceptance.md`.

## In scope (required for initial reconstruction)

The initial reconstruction is **complete** when the acceptance checklist can be fully checked, which implies:

- Implement **all** documented subsystems under `docs/technical/` (backend, frontend, config, deployment/runtime, testing, security, operability, contracts).
- Implement the minimum set of design behaviors needed to make the technical system coherent and playable, including:
  - world scale/units and chunking rules
  - settlements/villages as world anchors (spawn association + ≥ 1 interaction surface)
  - the core interaction loop (move, gather/craft, survival meters, basic combat)
  - the documented input/UX rules the server must enforce

If a feature is required by `docs/implementation/reconstruction_acceptance.md`, it is **in scope**.

## Planned / out of scope (for initial reconstruction)

These items are explicitly **not required** for the initial reconstruction unless and until they receive dedicated technical specifications (protocol + persistence + UI) and are added to acceptance/backlog docs.

- Parties, guilds/clans, multi-channel chat, friend/ignore lists
- Moderation tooling, reporting, audit logs, and live-ops dashboards
- Auction house / marketplace
- Weather + seasonal systems (when mentioned as optional)
- Large data volume targets (“100+ achievements”, “dozens of quest templates per tier”)

## Promoting planned items into scope

To promote a planned item into scope, update docs so reconstruction is unambiguous:

1. Add/extend protocol message shapes in `docs/technical/backend/server/protocol.md`.
2. Add/extend persistence specs under `docs/technical/backend/database/` and `docs/technical/backend/persistence/`.
3. Add/extend UI/runtime specs under `docs/technical/frontend/`.
4. Update `docs/implementation/reconstruction_acceptance.md` and `docs/implementation/todo/` accordingly.

Until those updates exist, planned items remain out of scope for the initial reconstruction.

