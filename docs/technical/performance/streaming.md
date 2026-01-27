# World Streaming Optimizations

## Chunk Lifecycle

- Load chunks only inside simulation radius.
- Freeze and serialize chunks outside the active radius.
- Limit active chunk count with LRU eviction.

## Network

- Send chunk deltas, not full snapshots.
- Batch entity updates per chunk.
- Cap update size per tick.

## CPU

- Skip AI for entities outside simulation radius.
- Use simple distance checks before expensive logic.
- Budget per-tick processing with hard caps.

## Persistence

- Write chunk deltas on a fixed interval.
- Avoid per-tick writes; coalesce changes.
