# Architecture Overview

## Structure

```
architecture/
├── README.md                  # This file
├── server-authoritative/      # Server authority design
│   └── README.md
├── networking/                # Network communication
│   └── README.md
├── simulation/                # Game simulation
│   └── README.md
├── persistence/               # Data persistence
│   └── README.md
└── frontend/                  # Client architecture
    └── README.md
```

## Core Architectural Principles

### 1. Server-Authoritative Design
- Single source of truth
- Client-server trust boundary
- Input validation and processing

### 2. Deterministic Simulation
- Fixed-rate tick loop
- Consistent state management
- Predictable behavior

### 3. Efficient Networking
- WebSocket primary transport
- Binary protocol
- Rate limiting and abuse prevention

### 4. Minimal Persistence
- PostgreSQL only
- Strategic data storage
- Performance optimization

### 5. Integrated Frontend
- TypeScript build-time compilation
- Rust-served assets
- No runtime Node.js dependency

## Key Documentation

- **Server Authority**: `server-authoritative/README.md`
- **Networking**: `networking/README.md`
- **Simulation**: `simulation/README.md`
- **Persistence**: `persistence/README.md`
- **Frontend**: `frontend/README.md`