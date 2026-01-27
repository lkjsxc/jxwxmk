# Achievements System

Achievements are data-driven and evaluated server-side.

## Data Model

- `AchievementId`: string key.
- `Achievement`: metadata (name, description, requirement, rewards).
  - `rewards` includes `xp` (integer) and stat bonuses.
- Player stores:
  - `achievements`: set of unlocked IDs.
  - `stat_bonuses`: map of stat -> additive bonus.
  - `xp`: accumulated player XP.

## Requirements

Supported requirement types:

- `Steps`
- `Kills`
- `Resources`
- `Crafts`
- `Structures`
- `WeaponMastery`
- `PlayerLevel`
- `EventParticipation`
- `ReputationTier`

## Evaluation

- Evaluated at the end of each tick for players with changed stats.
- Requirements are bounded and deterministic.

## Configuration

- Achievements load from `config/achievements.json`.
- Rewards apply immediately and persist with the player.
- XP rewards add to the player's XP and may trigger level-ups.
