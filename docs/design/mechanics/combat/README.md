# Combat Mechanics

PVP, PVE, and Resource Gathering systems.

## Damage Calculation
`Damage = BaseDamage * ToolMultiplier`

## Weapons & Tools
- **Hand**: 2 Base DMG.
- **Wood Pickaxe**: 4 Base DMG. 2x Multiplier against Rocks.
- **Stone Pickaxe**: 8 Base DMG. 3x Multiplier against Rocks.

## Creature Behavior (AI)
- **Aggression**: Hostile mobs (Wolves, Bears) will track the nearest player within their detection radius.
- **Attack**: Mobs deal damage to players upon contact or within a short range.
- **Cooldown**: Mobs have an internal attack cooldown (~1.0s).

## Player Interaction
- Players can attack each other.
- Shared structures can be interacted with by any player (unless locked).

