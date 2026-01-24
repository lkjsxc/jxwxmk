# Gameplay Systems

This directory contains documentation about game mechanics and systems.

## Current Structure

```
gameplay/
└── README.md          # This file (gameplay overview)
```

## Core Gameplay Pillars

### 1. Survival Mechanics
The foundation of the survival experience with essential player needs.

#### Health System
- **Base Health**: 100 points
- **Damage Sources**: Combat, environmental hazards, starvation
- **Regeneration**: Slow natural regeneration when well-fed
- **Death Penalty**: Drop inventory, respawn at safe location

#### Hunger System (Mandatory)
- **Hunger Meter**: 0-100 scale
- **Depletion Rate**: 1 point per 5 minutes (configurable)
- **Effects**:
  - 75-100: Well-fed (health regeneration bonus)
  - 50-75: Normal
  - 25-50: Hungry (movement speed penalty)
  - 0-25: Starving (health damage over time)
- **Food Sources**: Berries, meat, cooked food
- **Food Values**: Each food type provides different hunger restoration

#### Thirst System (Optional)
- **Thirst Meter**: 0-100 scale
- **Depletion Rate**: 1 point per 3 minutes (configurable)
- **Effects**:
  - 75-100: Hydrated (stamina regeneration bonus)
  - 50-75: Normal
  - 25-50: Thirsty (stamina penalty)
  - 0-25: Dehydrated (health damage over time)
- **Water Sources**: Rivers, lakes, rain collection

#### Stamina System
- **Stamina Meter**: 0-100 scale
- **Usage**: Sprinting, combat, heavy actions
- **Regeneration**: 5 points per second when resting
- **Effects**:
  - 75-100: Full stamina (normal capabilities)
  - 50-75: Tired (reduced sprint speed)
  - 25-50: Exhausted (cannot sprint)
  - 0-25: Fatigued (combat penalty)

### 2. Resource Gathering
The foundation of crafting and survival progression.

#### Resource Types

| Type | Source | Tools Required | Base Yield |
|------|--------|----------------|------------|
| Wood | Trees | Axe (better) / Hands | 5-10 logs |
| Stone | Rocks | Pickaxe (better) / Hands | 3-8 stones |
| Food | Bushes/Animals | Hands / Weapons | 1-5 items |
| Fiber | Plants | Hands / Knife | 2-6 fibers |
| Metal | Ore nodes | Pickaxe (tier 2+) | 1-3 ore |

#### Gathering Mechanics
- **Tool Efficiency**: Better tools yield more resources faster
- **Resource Depletion**: Resources have limited yields before exhaustion
- **Respawn Rules**:
  - Common resources: 5-15 minutes
  - Rare resources: 30-60 minutes
  - Animal respawns: Area-based with cooldowns

#### Gathering Actions
```typescript
interface GatheringAction {
  resource_type: string;  // "tree", "rock", "bush", etc.
  resource_id: string;    // Specific resource instance
  tool_used?: string;     // Tool ID if applicable
  duration: number;       // Action duration in ms
  yield: Array<{
    item_id: string;
    quantity: number;
  }>;
  experience_gained: number; // Skill experience
}
```

### 3. Crafting System
Progressive item creation from gathered resources.

#### Crafting Stations

| Station | Unlock Requirements | Crafting Tier |
|---------|---------------------|---------------|
| Hands | None | Basic |
| Workbench | 10 Wood, 5 Stone | Intermediate |
| Furnace | 20 Stone, 10 Wood | Advanced |
| Anvil | 30 Metal, 15 Stone | Expert |

#### Recipe Structure
```typescript
interface CraftingRecipe {
  id: string;
  name: string;
  station_required?: string; // Crafting station type
  ingredients: Array<{
    item_id: string;
    quantity: number;
  }>;
  output: {
    item_id: string;
    quantity: number;
  };
  crafting_time: number; // Seconds to craft
  skill_required?: string; // Required skill
  min_skill_level?: number; // Minimum skill level
  experience_gained: number; // Skill experience awarded
}
```

#### Example Recipes

**Basic Recipes (No Station):**
- Wooden Stick: 1 Wood → 2 Sticks
- Stone Axe: 3 Wood, 2 Stone → 1 Stone Axe
- Torch: 1 Stick, 1 Fiber → 1 Torch

**Intermediate Recipes (Workbench):**
- Wooden Pickaxe: 5 Wood, 3 Stone → 1 Wooden Pickaxe
- Backpack: 10 Fiber, 5 Wood → 1 Backpack (expands inventory)
- Campfire: 10 Stone, 5 Wood → 1 Campfire

**Advanced Recipes (Furnace):**
- Metal Bar: 3 Metal Ore, 1 Coal → 1 Metal Bar
- Metal Pickaxe: 3 Metal Bar, 2 Wood → 1 Metal Pickaxe
- Cooked Meat: 1 Raw Meat → 1 Cooked Meat (better hunger restoration)

### 4. Combat System
Player vs environment and player vs player interactions.

#### Combat Mechanics
- **Melee Combat**: Short-range, instant damage
- **Ranged Combat**: Projectile-based, travel time
- **Damage Types**: Slashing, piercing, blunt
- **Armor System**: Reduces incoming damage by type

#### Weapon Statistics
```typescript
interface WeaponStats {
  damage: number;        // Base damage per hit
  damage_type: string;   // "slashing", "piercing", "blunt"
  attack_speed: number;  // Attacks per second
  range: number;         // Attack range in units
  durability: number;    // Uses before breaking
  knockback: number;     // Knockback force
  // Ranged-specific
  projectile_speed?: number; // For ranged weapons
  projectile_gravity?: number; // Gravity effect
}
```

#### Armor System
```typescript
interface ArmorStats {
  defense: number;       // Base damage reduction
  defense_type: string;  // "light", "medium", "heavy"
  durability: number;    // Uses before breaking
  movement_penalty: number; // % movement speed reduction
  stamina_penalty: number; // % stamina usage increase
  // Type-specific resistances
  slashing_resist: number; // 0-1 resistance multiplier
  piercing_resist: number;
  blunt_resist: number;
}
```

#### Combat Calculation
```typescript
function calculateDamage(attacker: Entity, defender: Entity, weapon: Weapon): number {
  const baseDamage = weapon.damage;
  const armorReduction = defender.armor ? defender.armor.defense : 0;
  const typeResistance = defender.armor 
    ? getTypeResistance(defender.armor, weapon.damage_type) 
    : 1.0;
  
  const rawDamage = baseDamage - armorReduction;
  const finalDamage = Math.max(1, rawDamage * typeResistance);
  
  return applyRandomVariation(finalDamage, 0.9, 1.1); // ±10% variation
}
```

### 5. Progression System
Character development and unlockable content.

#### Experience and Levels
- **Global Experience**: Overall player progression
- **Skill-Based Experience**: Individual skill trees
- **Level Progression**: Unlocks new abilities and recipes

#### Skill System

| Skill | Description | Max Level | Effects |
|-------|-------------|-----------|---------|
| Gathering | Improves resource yield | 20 | +5% yield per level |
| Crafting | Reduces crafting time | 15 | -4% time per level |
| Combat | Increases damage | 25 | +2% damage per level |
| Survival | Reduces meter depletion | 20 | -3% depletion per level |
| Building | Improves structure health | 15 | +5% health per level |

#### Progression Rewards
- **Level Milestones**: Special rewards at key levels
- **Recipe Unlocks**: New crafting options
- **Stat Improvements**: Permanent character enhancements
- **Cosmetic Unlocks**: Visual customization options

### 6. World and Environment
The game world and its interactive elements.

#### Biome System

| Biome | Resources | Hazards | Features |
|-------|-----------|---------|----------|
| Forest | Trees, berries, animals | Wolves, bears | Dense foliage, caves |
| Desert | Cacti, rocks | Scorpions, heat | Oases, sandstorms |
| Mountains | Stone, metal ore | Falling rocks | High vantage points |
| Swamp | Reeds, mushrooms | Snakes, disease | Murky water, quicksand |
| Snow | Ice, rare minerals | Frostbite | Blizzards, auroras |

#### Day/Night Cycle
- **Cycle Duration**: 20 minutes (configurable)
- **Day Phase**: 8 minutes (safe, good visibility)
- **Night Phase**: 12 minutes (dangerous, poor visibility)
- **Effects**:
  - Night: Increased monster spawns, reduced visibility
  - Day: Better gathering yields, safer travel

#### Weather System
- **Weather Types**: Clear, rain, snow, storm
- **Effects**:
  - Rain: Fills water containers, extinguishes fires
  - Snow: Reduces movement speed, cold hazard
  - Storm: Lightning hazards, reduced visibility

### 7. Social Systems
Multiplayer interaction mechanics.

#### Team System
- **Team Formation**: Players can form temporary teams
- **Team Benefits**: Shared visibility, friendly fire protection
- **Team Size**: 2-4 players (configurable)
- **Team Chat**: Private communication channel

#### Clan System (Future)
- **Persistent Groups**: Long-term player associations
- **Shared Resources**: Clan storage and bases
- **Territory Control**: Clan-owned areas
- **Hierarchy**: Leader, officers, members

#### Trading System
- **Direct Trading**: Player-to-player item exchange
- **Trade Interface**: Secure trading UI
- **Trade Rules**:
  - Both players must confirm
  - Items held in escrow during trade
  - Time limit to prevent exploitation

## Gameplay Data Structures

### Player State
```typescript
interface PlayerState {
  id: string;
  name: string;
  position: { x: number; y: number };
  velocity: { x: number; y: number };
  
  // Survival meters
  health: number;       // 0-100
  hunger: number;       // 0-100
  thirst: number;       // 0-100
  stamina: number;      // 0-100
  
  // Progression
  level: number;
  experience: number;
  skills: Record<string, number>; // skill_id -> level
  
  // Inventory
  inventory: Array<{
    item_id: string;
    quantity: number;
    equipped: boolean;
  }>;
  
  // Equipment
  helmet?: string;
  chestplate?: string;
  leggings?: string;
  boots?: string;
  weapon?: string;
  tool?: string;
  
  // Status effects
  effects: Array<{
    type: string;         // "poison", "speed", "strength", etc.
    duration: number;     // Seconds remaining
    intensity: number;    // Effect strength
  }>;
  
  // Statistics
  playtime: number;      // Total seconds played
  deaths: number;
  kills: number;
  resources_gathered: Record<string, number>;
  items_crafted: Record<string, number>;
}
```

### World State
```typescript
interface WorldState {
  time: {
    total_seconds: number;
    day_night_cycle: number; // 0-1 (0=midnight, 0.5=noon)
    current_weather: string;
    weather_duration: number;
  };
  
  biomes: Array<{
    id: string;
    type: string;
    bounds: { x: number; y: number; width: number; height: number };
    temperature: number; // -1 (cold) to 1 (hot)
    humidity: number;    // 0 (dry) to 1 (wet)
  }>;
  
  resources: Array<Resource>;
  structures: Array<Structure>;
  entities: Array<Entity>; // Animals, monsters, etc.
  
  // Spawn rules
  resource_spawn_rules: Record<string, {
    min_respawn_time: number;
    max_respawn_time: number;
    max_per_area: number;
  }>;
  
  entity_spawn_rules: Record<string, {
    biome_types: string[];
    day_spawn_chance: number;
    night_spawn_chance: number;
    min_pack_size: number;
    max_pack_size: number;
  }>;
}
```

### Item Database
```typescript
interface ItemDefinition {
  id: string;
  name: string;
  description: string;
  type: "resource" | "tool" | "weapon" | "armor" | "consumable" | "structure";
  
  // Common properties
  stackable: boolean;
  max_stack: number;
  weight: number;       // For inventory capacity
  
  // Type-specific properties
  tool_properties?: {
    efficiency: number;  // Gathering speed multiplier
    durability: number;  // Uses before breaking
    gathering_types: string[]; // "wood", "stone", etc.
  };
  
  weapon_properties?: WeaponStats;
  armor_properties?: ArmorStats;
  
  consumable_properties?: {
    hunger_restore?: number;
    thirst_restore?: number;
    health_restore?: number;
    effects?: Array<{
      type: string;
      duration: number;
      intensity: number;
    }>;
  };
  
  structure_properties?: {
    health: number;
    build_time: number;
    maintenance_cost?: Record<string, number>; // Resources per day
    provides_shelter: boolean;
    storage_capacity?: number;
  };
  
  // Crafting info
  craftable: boolean;
  crafting_station?: string;
  crafting_time?: number;
}
```

## Gameplay Configuration

### Server Configuration Options

```typescript
interface GameplayConfig {
  // Survival settings
  hunger_depletion_rate: number; // Points per minute
  thirst_depletion_rate: number;
  health_regeneration_rate: number; // Points per second when well-fed
  
  // Combat settings
  base_player_damage: number;
  base_player_health: number;
  armor_effectiveness: number; // 0-1 multiplier
  
  // Gathering settings
  base_gathering_yield: Record<string, number>; // resource_type -> base yield
  tool_efficiency_multipliers: Record<string, number>; // tool_type -> multiplier
  
  // Crafting settings
  crafting_time_multipliers: Record<string, number>; // station_type -> multiplier
  
  // World settings
  day_night_cycle_duration: number; // Minutes for full cycle
  resource_respawn_ranges: {
    common: { min: number; max: number }; // Minutes
    uncommon: { min: number; max: number };
    rare: { min: number; max: number };
  };
  
  // Progression settings
  experience_curve: "linear" | "exponential" | "logarithmic";
  level_milestones: Record<number, string>; // level -> reward description
  
  // Social settings
  max_team_size: number;
  friendly_fire_enabled: boolean;
  shared_visibility_range: number; // Units
}
```

## Gameplay Balance Guidelines

### Resource Economy
- **Early Game**: Abundant basic resources, limited tools
- **Mid Game**: Balanced resource availability, tool requirements
- **Late Game**: Scarce high-tier resources, complex crafting

### Risk vs Reward
- **Safe Areas**: Low resource yield, minimal hazards
- **Danger Areas**: High resource yield, significant hazards
- **Night Time**: Increased rewards, increased risks

### Progression Curve
- **Early Levels**: Fast progression, frequent rewards
- **Mid Levels**: Steady progression, skill-based rewards
- **High Levels**: Slow progression, mastery rewards

### Combat Balance
- **Weapon Triangle**: Rock-paper-scissors balance between weapon types
- **Armor Trade-offs**: Defense vs mobility vs stamina
- **Environmental Factors**: Terrain advantages, weather effects

## Testing Gameplay Systems

### Unit Tests
- Resource gathering calculations
- Crafting recipe validation
- Combat damage formulas
- Progression experience curves

### Integration Tests
- Full gathering-to-crafting workflow
- Combat scenarios with different equipment
- Survival meter depletion and recovery
- Day/night cycle transitions

### Balance Testing
- Resource yield measurements
- Combat win-rate analysis
- Progression time tracking
- Economy inflation monitoring

## Future Gameplay Enhancements

### Planned Features
1. **Advanced Crafting**: Multi-step recipes, quality system
2. **Base Building**: Constructable player bases
3. **Vehicle System**: Rafts, carts, mounts
4. **Advanced AI**: Smarter NPC behaviors
5. **Seasonal Events**: Special limited-time content

### Experimental Ideas
1. **Skill Decay**: Skills degrade without use
2. **Permadeath Mode**: Hardcore survival option
3. **Procedural Quests**: Dynamically generated objectives
4. **Player Economy**: Crafting orders and markets
5. **Faction System**: Large-scale player organizations

## Related Documentation

- **Architecture Overview**: See `../architecture/README.md`
- **Protocol Specifications**: See `../protocol/README.md`
- **Operational Setup**: See `../operations/README.md`