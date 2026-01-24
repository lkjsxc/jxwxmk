# Database Persistence

## Schema
- **Accounts**: User sessions, auth tokens.
- **Inventory**: Player items (authoritative).
- **Equipment**: Current gear.
- **World Facts**: Biomes, resource nodes (minimal; no per-tick).

## Do Not Persist
- Transient state: Positions, health bars.
- High-frequency: Every tick physics.

## Implementation
- Migrations: Reproducible, committed.
- Queries: Minimal; use prepared statements.
- Checkpointing: Periodic saves for long-lived data.