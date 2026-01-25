# Backend Achievement Implementation

## Data Structures
- `AchievementId`: Enum for unique identification.
- `Achievement`: Struct containing metadata (name, description, required_progress).
- `PlayerAchievements`: Struct stored in `Player` component/state.
    - `unlocked`: Set of `AchievementId`.
    - `progress`: Map<AchievementCategory, u32> (e.g., MobsKilled: 50).

## Event System
- Use an `Observer` or `EventBus` pattern.
- **Events**: `MobKilled`, `ItemCrafted`, `StepTaken`, `ResourceGathered`.
- **Handler**: `AchievementSystem` listens to events and updates player progress.

## Persistence
- Achievements must be saved to the DB.
- Table `player_achievements` or JSON column in `players` table.
- Sync to client on login and on update.

## Network
- `AchievementUnlocked` packet sent via WebSocket.
- Payload: `id`, `name`, `stat_bonus`.
