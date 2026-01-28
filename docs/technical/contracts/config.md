# Config Contract (Schemas, Defaults, Validation)

Config is a **contract**, not a suggestion.
If config is ambiguous, implementation tends to drift and agents invent behavior.

References:
- `docs/technical/config/README.md`
- `docs/technical/config/files.md`
- `docs/technical/config/schemas/README.md`

## File set (required)

The canonical file set is defined in `docs/technical/config/files.md`.

## Schema-first

- Every config file has a documented schema under `docs/technical/config/schemas/`.
- The server validates each file independently.
- Recommended: include `version` in each file and validate it.

## Strictness

- Reject invalid JSON and wrong types.
- Reject unknown fields (prevents typos and silent drift).
- Validate numeric ranges (no NaN/Inf, no negative tick rates, etc.).

## Defaults and missing files

- Missing files fall back to documented defaults.
- Missing optional fields fall back to documented defaults.
- Defaults must be explicit in code, not implicit “zero values”.

## Priority and overrides

If multiple files can affect the same behavior:

- The precedence order must be explicit and documented.
- The server must log the resolved final values at startup (see: `docs/technical/operability/logging.md`).
