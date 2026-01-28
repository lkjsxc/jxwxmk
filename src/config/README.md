# Config Module

Loads and validates server configuration from JSON files.

## Responsibilities
- Define configuration structs matching `config/*.json` schemas.
- Load files with fallback to defaults.
- Validate logical constraints (e.g., min < max).

## Dependencies
- `protocol` (for shared ID types if needed).
