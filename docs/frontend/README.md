# Frontend Documentation

## Overview

The frontend is built with TypeScript using:
- **Vite**: Build tool and development server
- **Canvas API**: Game rendering
- **WebSocket**: Real-time communication

## Module Structure

```
/src/frontend/src
├── main.ts              # Entry point
├── game/               # Game logic
│   └── client.ts        # Network client
└── render/             # Rendering
    └── renderer.ts      # Game renderer
```

## Development

### Scripts

- `npm run dev`: Start development server
- `npm run build`: Build for production
- `npm run preview`: Preview production build
- `npm run lint`: Run ESLint
- `npm run typecheck`: Run TypeScript checker
- `npm run format`: Format code with Prettier

### Configuration

- `vite.config.ts`: Vite configuration
- `package.json`: Project dependencies and scripts

## Game Client

The `GameClient` class handles:
- WebSocket connection to backend
- Game state synchronization
- Input handling
- Network communication

## Rendering

The renderer uses Canvas API for:
- Game world rendering
- Player and entity drawing
- UI elements
- Animations