# Message Format Specifications

## Common Message Structure

```typescript
interface BaseMessage {
  protocol_version: string;  // "1.0.0"
  msg_type: string;         // Message type
  seq: number;              // Sequence number
  timestamp?: number;       // Client timestamp
}
```

## Message Types

### Handshake Messages
- `handshake`: Client → Server
- `handshake_ack`: Server → Client
- `reconnect`: Client → Server

### Gameplay Messages
- `input`: Client → Server
- `snapshot`: Server → Client
- `delta`: Server → Client
- `event`: Server → Client

### Control Messages
- `ping`: Bidirectional
- `pong`: Bidirectional
- `error`: Server → Client
- `disconnect`: Bidirectional

## Message Examples

### Handshake Message
```typescript
interface HandshakeMessage extends BaseMessage {
  msg_type: "handshake";
  client_info: {
    version: string;
    platform: string;
    capabilities: string[];
  };
}
```

### Input Message
```typescript
interface InputMessage extends BaseMessage {
  msg_type: "input";
  inputs: {
    movement?: {x: number, y: number};
    actions?: {primary: boolean, secondary: boolean};
    // ... other inputs
  };
}
```

### Snapshot Message
```typescript
interface SnapshotMessage extends BaseMessage {
  msg_type: "snapshot";
  server_tick: number;
  players: PlayerState[];
  world: WorldState;
}
```