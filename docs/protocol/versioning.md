# Protocol Versioning

## Rules
- Explicit `protocol_version` in every message.
- No backward compatibility: Clients must match server version.
- On change: Update version, test handshake, commit migration.

## Handshake
- Initial WS connect: Client sends version; server rejects if mismatch.
- Disconnect on version error.

## Evolution
- Add fields: New version.
- Remove fields: New version (break clients).