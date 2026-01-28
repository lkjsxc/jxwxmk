# Configuration

Configuration is split across multiple JSON files under `config/`.

## Location

- The runtime container mounts `/app/config`.
- The server loads all `*.json` files in that directory at startup.
- `config.json` at the repo root is deprecated in favor of the directory split.

## Files

See [Config File Map](files.md) for the canonical list and ownership.

## Schemas (authoritative)

Concrete JSON schemas (field names, types, defaults, validation rules) live under:

- [Config Schemas](schemas/README.md)

## Loading Rules

- Each file is validated independently.
- Defaults exist for optional fields; missing files fall back to defaults.
- Conflicts are resolved by explicit priority (server -> world -> balance -> system-specific).

## Strictness (recommended baseline)

- Reject unknown fields (prevents silent typos).
- Validate all numbers (finite; within documented ranges).
- Validate all identifiers as `snake_case` (see: `docs/technical/contracts/world_space.md` for general ID rules).
