# MMORPG Essentials

This checklist defines the minimum expected scope for a large-scale MMORPG survival game. Treat it as a baseline contract and cross-reference the design/technical docs below.

## Scope note (important)

This document mixes “MVP required” items with long-term MMO targets.

For **initial full reconstruction** of `src/`, the scope boundary is defined by:

- `docs/decisions/0001-reconstruction-scope.md`, and
- `docs/implementation/reconstruction_acceptance.md`.

Items that are described here but do not yet have dedicated technical specifications (protocol + persistence + UI) are treated as **planned** until promoted into scope by a decision/update.

## Identity + Sessions

- Persistent Player ID visible in the profile UI with a copy action.
- Device login by entering a Player ID; server issues a new session token.
- Single active session per player: new logins revoke existing sessions (`sessionRevoked`).
- Clear session state feedback in UI (connected, revoked, invalid).

## Platform Parity + Controls

- No gameplay feature split between PC and mobile; rules and UI affordances are the same.
- Tap/click = attack or gather; long-press/hold = interact.
- No on-screen A/B buttons; keyboard shortcuts remain as optional accelerators.
- Input parity is enforced by the server (client-side restrictions are mirrored server-side).

## Character + Progression

- Player level system with XP gates and tiered unlocks.
- Achievements grant XP on completion (mandatory XP source).
- Skill lines: gathering, crafting, combat, survival.
- Tech tiers for tools, weapons, armor, stations, and biomes.
- Reputation tiers tied to settlements/factions.

## Combat + Roles

- Melee and ranged combat (at least one of each).
- Status effects: bleed, slow, stun, poison.
- PvE threat/aggro rules and telegraphed attacks.
- Clear damage feedback with a 0.25s scale pulse on all damageable objects.

## World + PvE

- Persistent world with chunk streaming and biome variation.
- Resource nodes with respawn rules, level gating, and decay.
- Villages as anchors: spawn rules, stations, and safe-zone modifiers.
- PvE encounters: mobs, elites, bosses, roaming events.
- Lightweight instanced encounters (optional early, planned).

## Social + Group Play

- Parties with shared XP/quest credit and role hints.
- Guilds/clans with ranks, permissions, and shared storage.
- Chat channels: proximity, party, guild, system announcements.
- Friend/ignore lists and simple social discovery.

## Economy + Crafting

- Player-to-player trade and NPC vendors.
- Currency sources/sinks and crafting material sinks.
- Crafting stations by tier with shared projects.
- Marketplace/auction house (planned, can be phased).

## Quests + Narrative

- Quest types: gather, kill, escort, explore, craft, deliver.
- Daily/weekly rotations for retention.
- Quest-driven unlocks for biomes, recipes, and settlement services.

## Housing + Structures

- Buildable structures with durability, decay, and repair.
- Storage, crafting stations, defensive structures.
- Settlement contributions and upgrades.

## Live Ops + Safety

- Rate limiting and abuse detection.
- Player reporting, moderation tooling, audit logs.
- Telemetry for economy balance and progression pacing.
- Backups and recovery for persistent data.

## UX + Accessibility

- Unified controls across devices (tap/click, long-press/hold).
- Consistent interaction tooltips and targeting feedback.
- Readable UI scaling and color-contrast-safe palettes.

## Ops + Reliability

- CI builds the single runtime container on every push/PR.
- Database migrations are versioned and replayable.
- Runtime health checks for server + database.

## References

- Core loop: [design/core_loop.md](core_loop.md)
- Progression: [design/mechanics/progression.md](mechanics/progression.md)
- Achievements: [design/mechanics/achievements/README.md](mechanics/achievements/README.md)
- Villages: [design/world/settlements.md](world/settlements.md)
- UX/controls: [design/ux/controls/README.md](ux/controls/README.md)
- UI feedback: [design/ux/visuals_and_controls.md](ux/visuals_and_controls.md)
- Sessions + protocol: [technical/backend/server/http_ws.md](../technical/backend/server/http_ws.md)
