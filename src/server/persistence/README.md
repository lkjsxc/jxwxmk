# Persistence Crate

PostgreSQL persistence layer.

## Purpose

Provides durable storage for:
- Player accounts and sessions
- Player progression (levels, inventory, stats)
- Settlement state
- Chunk deltas

## Tables

- `players` - Player state and session tokens
- `settlements` - Settlement data
- `chunks` - Persisted chunk deltas

## Checkpoint Strategy

- Player state saved on disconnect and at intervals
- Chunk/settlement deltas checkpointed periodically
- No per-tick writes

## Migrations

Applied automatically at server startup.
