# System Architecture Overview

## High-Level Design
- **Server-Authoritative**: Server owns all gameplay state (positions, health, inventory). Clients are renderers + input devices.
- **Integrated Solution**: Rust handles simulation, WebSocket/HTTP, and static asset serving. TypeScript compiles client JS/CSS at build time.
- **Tick Loop**: Fixed-rate simulation (e.g., 30 Hz) using Tokio async. Processes inputs, advances world, publishes snapshots.
- **No Global State**: Use channels and ownership; bounded queues for inputs.
- **Simple Graphics**: Canvas 2D; focus on systems depth, not visuals.

## Components
- **Server**: Rust with Actix Web (handlers for WS/HTTP), world simulation task.
- **Client**: Browser JS from TS build; receives deltas, renders locally.
- **DB**: Postgres for persistence (accounts, inventory); no high-frequency ticks.
- **Network**: Binary protocol over WebSocket; versioning with protocol_version, seq, server_tick.

## Constraints
- Runtime: Rust + Postgres only.
- Build: Docker multi-stage for TS compilation.
- Security: Validate inputs; rate limit; no client trust for gameplay.