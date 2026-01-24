# Message Types and Formats

## Structure
- All messages: Binary with fields: protocol_version (u32), msg_type (enum), seq (u32), payload.
- Client → Server: Inputs (e.g., move, craft).
- Server → Client: Deltas (e.g., entity updates), snapshots (full state).

## Types
- `InputMove`: x, y deltas.
- `InputAction`: craft_id, target_pos.
- `DeltaEntity`: entity_id, pos, health.
- `Snapshot`: tick, entities[].

## Encoding
- Use serde for Rust; custom for client.
- Versioning: Bump on changes; no compat.