# System Architecture

## Overview

The game follows a client-server architecture with:
- **Backend**: Rust-based game server handling logic and state
- **Frontend**: TypeScript-based client for rendering and input
- **Database**: PostgreSQL for persistent data storage

## Component Diagram

```
┌───────────────────────────────────────────────────────┐
│                     Client (Browser)                  │
│                                                       │
│  ┌─────────────┐    ┌─────────────┐    ┌───────────┐  │
│  │   Renderer  │    │   Input     │    │  Network  │  │
│  │             │    │  Handling   │    │  Client   │  │
│  └─────────────┘    └─────────────┘    └───────────┘  │
└───────────────────────────────────────────────────────┘
                    │                          ▲
                    │ WebSocket               │ REST API
                    ▼                          │
┌───────────────────────────────────────────────────────┐
│                     Server (Rust)                     │
│                                                       │
│  ┌─────────────┐    ┌─────────────┐    ┌───────────┐  │
│  │ Game Logic  │    │ WebSocket   │    │  REST     │  │
│  │             │    │  Server     │    │  API      │  │
│  └─────────────┘    └─────────────┘    └───────────┘  │
│                                                       │
│  ┌─────────────────────────────────────────────────┐  │
│  │                 Database Layer                 │  │
│  │                                               │  │
│  │  ┌─────────────┐    ┌───────────────────────┐  │  │
│  │  │ PostgreSQL  │    │  Connection Pooling  │  │  │
│  │  │             │    │                       │  │  │
│  │  └─────────────┘    └───────────────────────┘  │  │
│  └─────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────┘
```

## Key Components

### Backend Services

1. **Game Logic Service**: Handles core game mechanics
2. **WebSocket Service**: Real-time communication with clients
3. **REST API Service**: HTTP endpoints for non-real-time operations
4. **Database Service**: PostgreSQL integration

### Frontend Components

1. **Game Renderer**: Canvas/WebGL based rendering
2. **Input Handler**: Keyboard/mouse input processing
3. **Network Client**: WebSocket communication
4. **UI System**: User interface components

## Data Flow

1. Client connects via WebSocket to game server
2. Server authenticates and adds player to game world
3. Game state updates are broadcast to all clients
4. Client input is sent to server for processing
5. Server updates game state and persists changes to database