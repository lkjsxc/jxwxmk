# Source Code

This directory contains all the source code for the game.

## Structure

```
/src
├── README.md              # Source code overview
├── backend/               # Rust backend
│   ├── Cargo.toml         # Rust project configuration
│   ├── src/               # Rust source files
│   └── ...
└── frontend/              # TypeScript frontend
    ├── package.json       # Node.js project configuration
    ├── src/               # TypeScript source files
    └── ...
```

## Backend

The Rust backend handles:
- Game logic and state management
- WebSocket connections for real-time gameplay
- REST API endpoints
- Database interactions with PostgreSQL
- Authentication and session management

## Frontend

The TypeScript frontend provides:
- Game client using canvas/WebGL
- User interface components
- Network communication with backend
- Input handling and rendering
- State management