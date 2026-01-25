# Visual Effects

Enhancements for game clarity and aesthetics.

## Interactable Outlines
To assist player recognition of interactable objects (Resources, Chests, Doors):
- **Condition**: Distance < InteractionRadius (e.g., 50px).
- **Effect**: Draw a 2px colored outline around the entity.
    - **Yellow**: Resources (Gatherable).
    - **White**: Structures (Interactable).
    - **Red**: Mobs (Attackable).

## Interpolation (Anti-Jitter)
To solve movement stutter caused by the difference between Server Tick Rate (20Hz) and Client Frame Rate (60Hz+):
- **Technique**: Linear Interpolation (Lerp).
- **Logic**:
    - Store `previous_world_state` and `current_world_state`.
    - Calculate `alpha` based on time since last packet.
    - `RenderPos = Lerp(PrevPos, CurrPos, alpha)`.
