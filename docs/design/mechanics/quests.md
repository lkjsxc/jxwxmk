# Quest System

Quests are structured tasks that players can perform for rewards and story progression.

## Quest Structure
- `id`: Unique identifier (string).
- `name`: Display name.
- `description`: Lore and instructions.
- `giver_id`: NPC ID who gives the quest.
- `objectives`: List of tasks to complete.
- `rewards`: Items, XP, or bonuses given upon completion.
- `state`: `NotStarted`, `InProgress`, `ReadyToTurnIn`, `Completed`.

## Objective Types
- **Gathering**: Collect a specific amount of items (e.g., "10 Wood").
- **Hunting**: Kill a specific number of mobs (e.g., "5 Wolves").
- **Dialogue**: Speak to a specific NPC.
- **Exploration**: Reach a specific coordinate or biome.

## Quest Flow
1. **Discovery**: NPC has an indicator (e.g., "!") if they have an available quest.
2. **Acceptance**: Player talks to NPC and accepts the quest via dialogue.
3. **Tracking**: Quest appears in the player's quest log. Objectives are tracked in real-time.
4. **Completion**: When objectives are met, the quest state becomes `ReadyToTurnIn`.
5. **Turn-in**: Player returns to the NPC (or another designated NPC) to receive rewards.

## Invariants
- A player can only have a specific quest active once.
- Rewards are only granted on server-side validation of objectives.
- Quest items for "Gathering" objectives are consumed upon turn-in if specified.
