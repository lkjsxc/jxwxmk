# Resource Gathering System

## Resource Types

| Type | Source | Tools | Base Yield |
|------|--------|-------|------------|
| Wood | Trees | Axe | 5-10 logs |
| Stone | Rocks | Pickaxe | 3-8 stones |
| Food | Bushes | Hands | 1-5 items |
| Fiber | Plants | Knife | 2-6 fibers |
| Metal | Ore | Pickaxe (Tier 2) | 1-3 ore |

## Gathering Mechanics

### Tool Efficiency
- Hands: 1x efficiency
- Basic tools: 2x efficiency
- Advanced tools: 3x efficiency
- Master tools: 4x efficiency

### Resource Depletion
- Each resource has limited uses
- Visual feedback on depletion
- Progressive yield reduction

### Respawn Rules
- Common: 5-15 minutes
- Uncommon: 15-30 minutes
- Rare: 30-60 minutes
- Epic: 60+ minutes

## Gathering Process

```rust
fn gather_resource(
    player: &Player,
    resource: &mut Resource,
    tool: Option<&Tool>
) -> GatheringResult {
    // Calculate efficiency
    // Determine yield
    // Apply skill bonuses
    // Deplete resource
    // Return gathered items
}
```

## Resource Distribution

### Biome-Based
- Forest: Wood, berries, animals
- Desert: Cacti, rare minerals
- Mountains: Stone, metal ore
- Swamp: Reeds, mushrooms

### Tier-Based
- Tier 1: Basic resources (wood, stone)
- Tier 2: Intermediate (metal, rare plants)
- Tier 3: Advanced (gemstones, exotic materials)

## Gathering Actions

### Action Types
- **Instant**: Quick collection (berries)
- **Timed**: Progress bar (mining)
- **Skill-based**: Mini-game (fishing)

### Multiplayer Interaction
- Resource contention
- Shared gathering
- Competition for rare resources
