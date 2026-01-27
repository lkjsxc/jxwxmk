# Configuration

Configuration is split across multiple JSON files under `config/`.

## Location

- The runtime container mounts `/app/config`.
- The server loads all `*.json` files in that directory at startup.
- `config.json` at the repo root is deprecated in favor of the directory split.

## Files

See [Config File Map](files.md) for the canonical list and ownership.

## Loading Rules

- Each file is validated independently.
- Defaults exist for optional fields; missing files fall back to defaults.
- Conflicts are resolved by explicit priority (server -> world -> balance -> system-specific).
