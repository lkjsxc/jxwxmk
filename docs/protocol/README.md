# Network Protocol

## Structure

```
protocol/
├── README.md              # This file
├── transport/             # Transport layer
│   └── README.md
├── messages/              # Message formats
│   └── README.md
├── serialization/         # Data serialization
│   └── README.md
├── security/              # Security protocols
│   └── README.md
└── examples/              # Protocol examples
    └── README.md
```

## Protocol Overview

### Transport Layer
- WebSocket primary transport
- HTTP fallback for static assets
- Connection management

### Message Format
- Common base structure
- Type-specific payloads
- Sequence numbers for ordering

### Serialization
- Binary protocol (production)
- JSON protocol (development)
- Version negotiation

### Security
- Authentication flow
- Message validation
- Rate limiting

## Key Documentation

- **Transport**: `transport/README.md`
- **Messages**: `messages/README.md`
- **Serialization**: `serialization/README.md`
- **Security**: `security/README.md`