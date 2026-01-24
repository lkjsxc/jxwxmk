# Survival Mechanics

## Health System

### Properties
- Base: 100 points
- Regeneration: 1 point per 5 seconds (when well-fed)
- Death penalty: Inventory drop, respawn delay

### Damage Sources
- Combat: Enemy attacks
- Environmental: Falling, hazards
- Starvation: Hunger depletion
- Dehydration: Thirst depletion

## Hunger System (Mandatory)

### Meter Properties
- Range: 0-100
- Depletion: 1 point per 5 minutes
- Effects by level:
  - 75-100: Well-fed (+health regen)
  - 50-75: Normal
  - 25-50: Hungry (-movement speed)
  - 0-25: Starving (health damage)

### Food Sources
- Berries: 10 hunger, instant
- Meat: 25 hunger, requires cooking
- Cooked food: 40 hunger, bonuses

## Thirst System (Optional)

### Meter Properties
- Range: 0-100
- Depletion: 1 point per 3 minutes
- Effects by level:
  - 75-100: Hydrated (+stamina regen)
  - 50-75: Normal
  - 25-50: Thirsty (-stamina)
  - 0-25: Dehydrated (health damage)

### Water Sources
- Rivers: Safe, unlimited
- Lakes: Safe, unlimited
- Rain: Limited collection
- Wells: Player-built, safe

## Stamina System

### Meter Properties
- Range: 0-100
- Regeneration: 5 points per second (when resting)
- Usage: Sprinting, combat, heavy actions

### Effects by Level
- 75-100: Full stamina
- 50-75: Tired (-sprint speed)
- 25-50: Exhausted (no sprint)
- 0-25: Fatigued (-combat effectiveness)

## Implementation

```rust
struct SurvivalMeters {
    health: f32,
    hunger: f32,
    thirst: f32,
    stamina: f32,
}

fn update_meters(player: &mut Player, delta: f32) {
    // Deplete hunger/thirst over time
    // Apply effects based on levels
    // Handle regeneration
}
```