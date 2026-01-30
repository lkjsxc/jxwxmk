# World State

The server owns a single `World` struct with chunked storage.

## World Fields

- `seed`: global world seed
- `chunks`: `HashMap<ChunkCoord, Chunk>`
- `players`: `HashMap<PlayerId, PlayerState>`
- `active_chunks`: bounded set of loaded chunks
- `interest_sets`: per-player set of visible chunk coords

## Chunk

- `coord`: `(cx, cy)`
- `biome_id`
- `resources`, `mobs`, `structures`, `npcs`
- `settlement_id` (optional)
- `cooldowns` (respawn timers, event states)

## Serialization

- Chunks are streamed to clients as add/remove/update events.
- Entities serialize with stable IDs scoped to their chunk.

## Broadcast Rules

- Clients receive only entities inside their interest set.
- Updates are delta-based (no full-world snapshots).
- Far chunks are frozen and not ticked until reactivated.

## Interest set + streaming diffs (required)

Chunk streaming requires the server to track two related sets per connected player:

- **interest set**: which chunk coords should be visible now (based on player position and `world.view_radius_chunks`).
- **loaded set**: which chunk coords the client currently has in memory (the server’s view of what it has sent via `chunkAdd` minus `chunkRemove`).

On each tick (or at a bounded cadence), the server computes diffs:

- `to_add = interest - loaded` → send `chunkAdd` for each coord (full chunk snapshot).
- `to_remove = loaded - interest` → send `chunkRemove` for each coord.

Entity changes within loaded chunks are sent via `entityDelta`.
