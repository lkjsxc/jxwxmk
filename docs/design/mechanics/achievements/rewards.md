# Rewards & Notifications

## Stat Bonuses (Current)

Achievements grant additive bonuses recorded in `player.stat_bonuses`.

- `speed`: used in movement.
- `damage`: used in attack damage.
- `gather`: used in resource damage.
- `max_hp` and `craft`: currently stored but **not applied** by gameplay logic.

## Notifications

- Unlocks trigger a toast notification on the client.
- Duration: ~3 seconds.
- Content: title + achievement name.
