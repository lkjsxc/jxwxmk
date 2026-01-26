# NPCs (Non-Player Characters)

NPCs are neutral or friendly entities that players can interact with to progress in the game, trade resources, or learn about the world.

## NPC Types

### Elder
- **Role**: Quest giver and world guide.
- **Location**: Usually found in the center of Villages.
- **Interactions**: Dialogue, Quests.

### Merchant
- **Role**: Trading items and resources.
- **Location**: Markets in Villages.
- **Interactions**: Trading (Buy/Sell).

### Guard
- **Role**: Protecting Villages from Mobs.
- **Location**: Village perimeters.
- **Interactions**: Minimal dialogue, combat support.

## NPC Properties
- `id`: Unique identifier.
- `npc_type`: Elder, Merchant, Guard, etc.
- `name`: Display name.
- `x`, `y`: Position.
- `health`: Current health.
- `dialogue_tree`: Current state of conversation with players.
- `inventory`: Items available for trade (for Merchants).

## Interactions
- **Dialogue**: Text-based conversation with branching choices.
- **Quests**: Tasks given by NPCs (e.g., "Collect 10 Wood", "Kill 5 Wolves").
- **Trading**: Exchange of resources using a currency or barter system.
