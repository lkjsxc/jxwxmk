# Client

The client is a "dumb" terminal that renders state and captures input.

## Responsibilities

- **Rendering**: Draw the world state received from the server via WebSocket.
- **Input**: Capture mouse/keyboard/touch events and send them to the server.
- **UI**: Overlay HTML interfaces for inventory, crafting, etc.

## Modules

- `index.ts`: Entrypoint and main loop.
- `net.ts`: WebSocket handling and protocol parsing.
- `input.ts`: InputManager (Unified Input).
- `render.ts`: Canvas2D rendering logic.
- `ui.ts`: HTML UI state management.
