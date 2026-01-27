# Combat Mechanics

Combat includes melee and ranged options with server-side resolution.

## Damage Model

`Damage = WeaponDamage * (1 + stat_bonus + level_bonus)`

- Base values are defined in `config/balance.json`.
- Level bonus scales with item and player tier.

## Weapon Classes

- **Melee**: axes, swords, maces, spears.
- **Ranged**: bows, crossbows, thrown weapons.
- **Magic-lite**: staves and scrolls with cooldown-based effects.

## Status Effects

- Bleed, slow, poison, burn, chill, stun.
- Resistance values derive from armor and survival skills.

## PvP Rules

- Safe zones around settlement cores disable PvP.
- PvP is enabled in open world and certain event regions.
- Guards respond to hostile actions within settlement bounds.

## AI Combat

- Mobs have aggro radius, leash range, and threat table.
- Elites and bosses use telegraphed attacks.
