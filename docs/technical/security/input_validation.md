# Input validation rules

Validation must be strict and centralized so gameplay code can assume invariants.

## General rules

- Reject unknown message types.
- Reject missing required fields.
- Reject unknown fields for inbound messages (recommended: deny unknown fields).
- Reject NaN/Inf and non-finite numbers.
- Clamp or reject extreme magnitudes (world coordinate sanity bounds).
- Enforce string length limits (names, IDs).

## Action targeting (`aim`)

For any message using `aim`:

- Require `aim` when `attack` or `interact` is true.
- Validate `aim` is within max interaction range of the player.
- Validate `aim` does not exceed absolute world bounds.

## Inventory indices

For `slot` / `swapSlots`:

- Validate indices are within `[0, inventory_size)`.
- Reject invalid indices with a structured `error` (do not panic).

## Failure reporting

For validation failures:

- Do not mutate world state.
- Prefer sending a structured `error` to the client (unless the connection is abusive, in which case disconnect).
