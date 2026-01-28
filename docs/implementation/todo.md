# Implementation TODOs

This file tracks outstanding implementation tasks for the reconstruction.

## Gameplay Systems
- [ ] Crafting System (Recipe validation, inventory consumption)
- [ ] Spawning System (Resource/Mob population)
- [ ] AI System (Mob behavior)
- [ ] Combat System (Attack logic, damage calculation)
- [ ] Quest System (Progress tracking)
- [ ] Achievement System (Stat tracking)

## Infrastructure
- [ ] Implement DB lookup in `ws_route` (currently authenticates all tokens)
- [ ] Implement DB lookup in `claim_session` (currently returns random token)
- [ ] Implement `active_chunks` management in World
- [ ] Implement `Chunk` persistence (load/save to DB)
- [ ] Implement `Player` persistence (save on disconnect/interval)

## Frontend
- [ ] Reconstruct `src/client/*` source files (only READMEs exist currently)
- [ ] Implement Canvas rendering loop
- [ ] Implement Input manager