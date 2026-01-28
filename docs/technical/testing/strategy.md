# Strategy + layers

## Goals

- Catch logic bugs with deterministic unit tests.
- Catch integration drift (protocol/persistence/config) with containerized tests.
- Prevent “works on host” paths: Docker is canonical.

## Test layers

### 1) Unit tests (pure logic)

- Systems and helpers should be deterministic and unit-tested:
  - survival tick math
  - crafting consumption/output
  - placement validation
  - barrier safe-zone rules

### 2) Integration tests (DB + protocol)

Containerized tests must cover:

- migrations apply successfully
- session claim rotates token and revokes old session
- WebSocket handshake (`welcome`) and spawn flow
- config loading from `/app/config`

### 3) Smoke tests (runtime image)

- Build runtime image.
- Run container.
- Hit `/health` and load `/` (embedded static).

## What not to test

- Rendering correctness pixel-by-pixel (keep the client lightweight).
- Non-deterministic timing behavior (avoid flaky tests).
