# 03 — Configuration (`config/*.json` + loader)

Goal: implement config files + a strict loader so the server is data-driven and reproducible.

References:
- `docs/technical/config/README.md`
- `docs/technical/config/files.md`
- `docs/technical/config/schemas/README.md`
- `docs/technical/backend/game/*` (systems consume config values)

## A) Create the config file set (minimal seed values)

Files required by `docs/technical/config/files.md`:

- `server.json`
- `world.json`
- `balance.json`
- `survival.json`
- `crafting.json`
- `spawning.json`
- `biomes.json`
- `settlements.json`
- `economy.json`
- `quests.json`
- `achievements.json`

Tasks:

- [x] Create `config/` directory containing the full file set above.
- [x] Provide minimal-but-valid JSON in each file (seed values sufficient to run the game loop).
- [x] Ensure each file includes `version` and matches its schema under `docs/technical/config/schemas/`.
- [x] Reject unknown fields (implementation should fail fast on startup).
- [x] Ensure config values cover the parameters explicitly referenced by systems docs:
  - tick rate (`server.tick_rate`)
  - survival rates (`survival.*`)
  - barrier core parameters (`settlements.barrier.base_range_wu`, `settlements.barrier.level_multiplier_wu`)
  - crafting recipes (seed at least the items listed in `docs/design/mechanics/crafting/recipes.md`)

## B) Implement the config loader (Rust)

- [x] Server loads all `*.json` from `/app/config` at startup.
- [x] Each file is parsed and validated independently; missing files fall back to defaults.
- [x] Validation rejects malformed JSON, wrong types, NaNs/Infs, and out-of-range values.
- [x] Validation rejects unknown fields (prevents silent config drift).
- [x] Conflicts are resolved by explicit priority (as documented in `docs/technical/config/README.md`).

## C) Config usage wiring

- [x] `server.json` values are applied (ports, tick rate, rate limits).
- [x] `world.json` values are applied (chunk size, streaming radii, world seed).
- [x] Systems read config values (no "loaded then ignored" configs).

## Done when

- [x] Server starts with only `/app/config/*.json` mounted and does not crash.
- [x] Config-driven values demonstrably affect behavior (tick rate, survival decay, etc.).
