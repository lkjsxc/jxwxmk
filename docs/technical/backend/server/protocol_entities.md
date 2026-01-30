# Protocol — Entity Shapes

This file defines the public entity shapes used by `chunkAdd` snapshots and per-tick `entityDelta` updates.

See also: `protocol.md`.

## Entity Snapshot / Update

```json
{ "id": "e1", "kind": "resource", "subtype": "tree", "x": 12.5, "y": 9.0, "hp": 30.0, "max_hp": 30.0, "level": 2, "name": null, "range": null }
```

- `kind`: `player | resource | mob | structure | npc` (players arrive via `entityDelta`).
- `subtype`: resource/mob/structure/NPC type identifier.
- `hp`, `max_hp`, `level`, `name`, `range` are optional and omitted when irrelevant.
- Private player-only state (inventory, vitals, quests, achievements, etc.) is synchronized separately via `playerUpdate`.

## Entity Removal

```json
{ "id": "e1", "kind": "resource" }
```
