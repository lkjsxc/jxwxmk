# Recreate Source Plan

This plan is for reconstructing the entire source tree from documentation.

## Preconditions

- `src/` is deleted.
- Documentation in `docs/` is authoritative.
- Runtime must be a single container that runs Rust server + PostgreSQL.
- Reconstruction scope and “done” are defined by:
  - `docs/decisions/0001-reconstruction-scope.md`
  - `docs/implementation/reconstruction_acceptance.md`
- Implementation tasks are tracked in:
  - `docs/implementation/todo/README.md`

## Plan

1. **Read Policies and Constraints**
   - `docs/policy/INSTRUCT.md`
   - `docs/decisions/0001-reconstruction-scope.md`
   - `docs/implementation/reconstruction_acceptance.md`
   - `docs/implementation/todo/README.md`

2. **Recreate Repo Layout**
   - Recreate `src/` with required README.md files.
   - Restore the runtime Dockerfile and entrypoint.

3. **Rebuild Backend**
   - Implement Actix Web server and WebSocket session actors.
   - Implement game engine actor with fixed tick loop and world broadcast.
   - Implement entities and systems per `docs/technical/backend/game/*`.
   - Implement config loader and apply config values in systems.

4. **Rebuild Frontend**
   - Implement `src/client` TypeScript code per `docs/technical/frontend/*`.
   - Build Canvas renderer, UI, input, and WebSocket client.
   - Ensure client assets are bundled into `src/static`.

5. **Wire Build + Runtime**
   - Multi-stage Docker build: Node -> Rust -> Runtime.
   - Entrypoint starts Postgres, creates DB, and launches server.

6. **Add Tests**
   - Unit tests for crafting, survival, achievements, quests.
   - Integration tests for protocol handshake and config loading.

7. **Validate**
   - Build Docker image.
   - Run container and confirm `/health` and `/`.
   - Check off every item in `docs/implementation/reconstruction_acceptance.md`.

## Output Targets

- `src/` fully restored with documented files.
- `src/runtime/Dockerfile` and `src/runtime/entrypoint.sh` restored.
- `src/client` and `src/static` restored.
