# Message Types and Formats

## Structure
- All messages: Binary with fields: protocol_version, msg_type, seq, payload.
- Client → Server: Inputs (e.g., move, craft).
- Server → Client: Snapshots (world state).

## Types
- `Input`: player_id, action (string), data (binary).
- `Snapshot`: server_tick, world_state (binary serialized world).

## Encoding
- Use bincode in Rust; manual parse in TS.
- Versioning: Check in handler; disconnect on mismatch.