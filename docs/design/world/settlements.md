# Settlements + Barrier Cores

Settlements are generated around barrier cores and define safe zones, services, and early progression.

## Settlement Tiers

- **Outpost** (Level 1 core): 3-6 NPCs, basic trade, starter quests.
- **Village** (Level 2-3 core): crafting stations, storage, guards, regional quests.
- **Town** (Level 4-6 core): market district, advanced crafting, faction hubs.
- **City** (Level 7-10 core): multiple districts, elite quests, world events.

## Generation Pattern

- Barrier core is the geometric center of the settlement.
- Concentric rings expand outward:
  1. **Core ring**: core, respawn shrine, bulletin board.
  2. **Market ring**: traders, storage, crafting stations.
  3. **Housing ring**: villager homes, inns, taverns.
  4. **Outer ring**: farms, stables, guard posts, gates.

## Ruleset Differences

- **Safe zone** inside core radius (no PvP, guards intercept threats).
- **Spawn/respawn binding** to the nearest core a player has attuned to.
- **Guard escalation**: warnings, then forced teleport out or stun if hostile.

## Scale Notes

- Safe-zone radius is measured in **world units (wu)** (see: `scale_and_chunks.md`).
- The safe-zone radius must be large enough to cover at least the **Core ring** and most of the **Market ring** so the settlement feels meaningfully protected.

## Villager Variety

- Each settlement mixes roles (traders, crafters, quest givers, guards, healers).
- Role mix scales with tier, biome, and faction alignment.

## Barrier Core Coupling

- Core level controls safe-zone radius and maximum settlement tier.
- Core integrity affects services (if damaged, services degrade until repaired).
