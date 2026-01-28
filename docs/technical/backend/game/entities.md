# Entities

## Player

Fields:

- `id`, `token`, `username`
- `x`, `y`, `chunk`
- `health`, `hunger`, `temperature`, `thirst?`
- `inventory` (slots + stacks)
- `active_slot`
- `stats` (steps, kills, crafts, gathers, deaths)
- `level`, `xp`
- `achievements` (set of unlocked IDs)
- `quests` (active + completed)
- `reputation` (per settlement/faction)
- `spawned` (bool)

Synchronization boundary:

- Public world visibility (position and other visible entity fields) is streamed via `entityDelta`.
- Private player-only state (inventory, vitals, quests, achievements, progression) is sent to the session owner via `playerUpdate`.

See:

- `docs/technical/backend/server/protocol.md` (`playerUpdate`, `entityDelta`)
- `docs/technical/contracts/protocol.md` (public vs private state)

## Item

- `kind`: enum
- `amount`
- `level`, `quality`, `xp` (tools/weapons)

## Resource Node

- `r_type` (tree, stone, ore, plant)
- `level` (L1-L10)
- `amount`
- `respawn_at`

## Mob

- `m_type` (fauna, predator, monster, boss)
- `level`, `health`
- `aggro_state`, `leash_origin`

## Structure

- `s_type` (wall, station, settlement)
- `tier`, `health`, `owner_id`

## NPC

- `role`, `faction`, `name`
- `inventory`, `service_catalog`
- `reputation_gate`

## BarrierCore

- `level`, `base_range`, `faction`
- `integrity`
