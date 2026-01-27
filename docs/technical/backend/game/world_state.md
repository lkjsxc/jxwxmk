# World State

The server owns a single `World` struct that is cloned and broadcast each tick.

## World Fields

- `width`, `height` (f64): world bounds.
- `players`: `HashMap<Uuid, Player>`
- `resources`: `HashMap<Uuid, Resource>`
- `mobs`: `HashMap<Uuid, Mob>`
- `structures`: `HashMap<Uuid, Structure>`
- `npcs`: `HashMap<Uuid, Npc>`
- `barrier_cores`: `HashMap<Uuid, BarrierCore>`

## Serialization

- HashMaps serialize to JSON objects keyed by UUID strings.
- Each value is the serialized entity struct.

## Broadcast Rules

- Only spawned players are included in world updates.
- All other entity maps are broadcast as-is (full snapshot each tick).
