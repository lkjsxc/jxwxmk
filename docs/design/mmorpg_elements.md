# MMORPG Essentials

This checklist captures core elements expected of an MMORPG-scale survival game. It complements the existing design documents and should be treated as a baseline roadmap.

## Identity + Account

- Persistent Player ID (copyable in UI).
- Session token rotation and single active session per player.
- Device login by entering Player ID (server revokes old session).

## Character + Progression

- Player levels with XP (achievements grant XP on completion).
- Skill lines (gathering, crafting, combat, survival).
- Gear tiers, item levels, and recipe unlock gates.
- Reputation tiers tied to settlements/factions.

## Combat + Roles

- Melee and ranged combat (at least one of each).
- Status effects (bleed, slow, stun, poison).
- Threat/aggro rules for PvE.
- Combat telegraphs and damage feedback (scale pulse).

## World + PvE

- Persistent world with chunk streaming and biome variation.
- Resource nodes with respawn rules and level gating.
- Villages as core anchors (spawns, stations, ruleset differences).
- PvE encounters: mobs, elites, bosses, roaming events.
- Dungeons or instanced encounters (lightweight, optional early).

## Social + Group Play

- Parties with shared XP/quest credit.
- Guilds/clans with ranks, permissions, shared storage.
- Chat: proximity, party, guild, and system announcements.
- Friend/ignore lists (optional early but planned).

## Economy + Crafting

- Player-to-player trade and NPC vendors.
- Currency sources and sinks.
- Crafting stations by tier, with shared projects.
- Marketplace/auction house (planned, can be phased).

## PvP + Risk Management

- PvP flagging and safe zones (villages).
- Death penalties tuned for survival (drop/decay/repair).
- Anti-griefing rules (spawn protection, cooldowns).

## Quests + Narrative

- Quest types: gather, kill, escort, explore, craft, deliver.
- Daily/weekly quest rotations for retention.
- Quest-driven unlocks for biomes and recipes.

## Housing + Structures

- Buildable structures with durability and decay rules.
- Storage chests, crafting stations, defensive structures.
- Settlement contributions and upgrades.

## Live Ops + Safety

- Rate limiting and abuse detection.
- Player reporting and moderation tooling.
- Telemetry for economy balance and progression pacing.
- Backups and recovery for persistent data.

## UX + Accessibility

- Unified controls across devices (tap/click, long-press/hold).
- Tooltips for context actions and hotbar state.
- Clear session and login state feedback.
