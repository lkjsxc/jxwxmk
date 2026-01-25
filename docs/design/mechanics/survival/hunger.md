# Hunger Mechanics

## Decay Logic

- **Base Decay**: -0.1 Hunger / second (Reduced for better experience).
- **Action Penalty**: 
    - Walking: -0.02 / sec
    - Attacking/Gathering: -0.2 / action
- **Cold Penalty**: If Temperature < 20, Decay * 1.5x.

## Effects

- **Starvation**: If Hunger <= 0:
    - HP Decay: -5 HP / second.
- **Regeneration**: If Hunger >= 90:
    - HP Regen: +1 HP / second.

## Restoration

- **Berry**: +5 Hunger.
- **Cooked Meat**: +30 Hunger.
- **Bread**: +50 Hunger.
