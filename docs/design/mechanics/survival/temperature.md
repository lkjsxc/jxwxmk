# Temperature Mechanics

## Environmental Factors

- **Base Ambient**: 20°C (Neutral).
- **Night**: -20°C offset.
- **Winter Biome**: -40°C offset.
- **Water**: Wetness factor increases heat loss.

## Player State (Internal Temp)

The player's internal temperature moves towards the Ambient Temp at a rate of 1°C/sec.

## Modifiers

- **Campfire**: +50°C (Radius 200px).
- **Fur Clothing**: Reduces heat loss rate by 50%.
- **Roof**: Ignores "Night" cold offset.

## Consequences

- **Freezing**: If Internal Temp < 0:
    - HP Decay: -2 HP / sec.
    - Movement Speed: -30%.
- **Overheating**: If Internal Temp > 80:
    - Thirst Decay: * 2.0x.
