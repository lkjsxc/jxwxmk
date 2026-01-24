# Networking Model

## Protocol
- **WebSocket**: Real-time gameplay (input events, state deltas).
- **HTTP**: Login/session, asset delivery, matchmaking.
- **Binary**: Compact; explicit versioning.

## Messages
- Every message: `protocol_version`, `msg_type`, `seq` (client seq), `server_tick`.
- Input: Movement, actions.
- Delta: Position/health changes.
- Snapshot: Full state sync.

## Implementation
- Bounded queues: WS handlers enqueue events.
- Tick loop: Process inputs, publish to clients.
- Reconciliation: Clients use server_tick for prediction.