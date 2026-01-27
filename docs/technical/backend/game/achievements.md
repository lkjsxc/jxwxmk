# Achievements System

Achievements are evaluated server-side during gameplay and unlock permanent stat bonuses stored on the player.

## Data Model

- `AchievementId`: enum of fixed identifiers.
- `Achievement`: metadata (name, description, requirement, stat bonus).
- Player stores:
  - `achievements`: set of unlocked IDs (as strings).
  - `stat_bonuses`: map of stat name -> additive multiplier.

## Requirements

Each achievement has one requirement type:

- `Steps(u32)`
- `Kills(u32)`
- `Resources(u32)`
- `Crafts(u32)`
- `Structures(u32)`
- `ToolLevel(u32)`

Serialized JSON uses a tagged structure:

```json
{ "type": "Steps", "value": 1000 }
```

## Evaluation

- Called after input handling and after mob damage each tick.
- The highest tool level in inventory is used for `ToolLevel` achievements.
- Special case: `Pacifist` requires zero kills.

## Current Achievement List

- **NoviceWalker**: 1,000 steps, +1% speed
- **MarathonRunner**: 100,000 steps, +5% speed
- **FirstBlood**: 1 kill, +1% damage
- **MonsterHunter**: 100 kills, +2% damage
- **Slayer**: 1,000 kills, +5% damage
- **Lumberjack**: 100 resources, +2% gather
- **Deforestation**: 1,000 resources, +5% gather
- **Miner**: 100 resources, +2% gather
- **DeepDriller**: 1,000 resources, +5% gather
- **ApprenticeSmith**: 10 crafts, +2% craft (stored, not applied)
- **MasterSmith**: 1,000 crafts, +5% craft (stored, not applied)
- **Builder**: 50 structures, +5 max_hp (stored, not applied)
- **Architect**: 500 structures, +20 max_hp (stored, not applied)
- **SeasonedVeteran**: Tool level 5, +5% damage
- **LegendarySmith**: Tool level 10, +10% damage
- **Pacifist**: 5,000 steps and 0 kills, +10 max_hp (stored, not applied)
- **ResourceTycoon**: 5,000 resources, +10% gather

## Network

Unlocked achievements are pushed to the client as `achievement` messages.
