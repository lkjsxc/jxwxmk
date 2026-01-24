# TypeScript Client

Browser-based client implementation using TypeScript.

## Structure

```
client/
├── README.md              # This file
├── package.json           # npm project configuration
├── tsconfig.json          # TypeScript configuration
├── src/
│   ├── index.ts           # Main entry point
│   ├── game/              # Game logic
│   ├── network/           # Network communication
│   ├── ui/                # User interface
│   ├── rendering/         # Rendering system
│   ├── input/             # Input handling
│   ├── state/             # State management
│   ├── assets/            # Asset management
│   ├── utils/             # Utilities
│   └── types/             # Type definitions
├── public/                # Static assets
│   ├── index.html         # HTML entry point
│   ├── styles/            # CSS styles
│   └── assets/            # Static assets
└── tests/                 # Client tests
```

## Key Components

### Main Entry Point
- **File**: `src/index.ts`
- **Responsibilities**:
  - Initialize game systems
  - Set up rendering loop
  - Establish network connection
  - Handle browser events

### Game Logic
- **Module**: `game/`
- **Files**:
  - `index.ts` - Main game module
  - `world.ts` - World representation
  - `player.ts` - Player controller
  - `entities.ts` - Entity management
  - `systems.ts` - Game systems

### Network Communication
- **Module**: `network/`
- **Files**:
  - `index.ts` - Network module
  - `websocket.ts` - WebSocket client
  - `messages.ts` - Message handling
  - `protocol.ts` - Protocol implementation
  - `reconnection.ts` - Reconnection logic

### User Interface
- **Module**: `ui/`
- **Files**:
  - `index.ts` - UI module
  - `hud.ts` - Heads-up display
  - `inventory.ts` - Inventory interface
  - `crafting.ts` - Crafting interface
  - `chat.ts` - Chat system
  - `menus.ts` - Game menus

### Rendering System
- **Module**: `rendering/`
- **Files**:
  - `index.ts` - Rendering module
  - `canvas.ts` - Canvas management
  - `sprites.ts` - Sprite rendering
  - `tiles.ts` - Tile rendering
  - `camera.ts` - Camera system
  - `particles.ts` - Particle effects

### Input Handling
- **Module**: `input/`
- **Files**:
  - `index.ts` - Input module
  - `keyboard.ts` - Keyboard input
  - `mouse.ts` - Mouse input
  - `gamepad.ts` - Gamepad input
  - `bindings.ts` - Key bindings

### State Management
- **Module**: `state/`
- **Files**:
  - `index.ts` - State module
  - `game.ts` - Game state
  - `player.ts` - Player state
  - `world.ts` - World state
  - `sync.ts` - State synchronization

### Asset Management
- **Module**: `assets/`
- **Files**:
  - `index.ts` - Assets module
  - `loader.ts` - Asset loader
  - `cache.ts` - Asset cache
  - `sprites.ts` - Sprite management
  - `sounds.ts` - Sound management

## Development Setup

### Prerequisites
- Node.js 18+
- npm 9+
- TypeScript 5+

### Getting Started

```bash
# Initialize project
npm init -y

# Install TypeScript
npm install typescript --save-dev

# Install dependencies
npm install @types/node --save-dev
npm install ws @types/ws
npm install uuid
npm install howler  # For audio

# Create tsconfig.json
npx tsc --init
```

### Example `tsconfig.json`

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "moduleResolution": "node",
    "outDir": "../assets/js",
    "rootDir": "src",
    "sourceMap": true,
    "declaration": true,
    "lib": ["DOM", "ESNext"]
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "tests"]
}
```

### Building

```bash
# Development build with watch
npm run dev

# Production build
npm run build

# Type checking
npx tsc --noEmit
```

### Testing

```bash
# Run tests
npm test

# Run tests with coverage
npm run test:coverage
```

## Architecture Patterns

### Game Loop
```typescript
class GameLoop {
    private lastTime: number = 0;
    private lag: number = 0;
    private msPerUpdate: number = 1000 / 60; // 60 FPS
    
    start() {
        this.lastTime = performance.now();
        requestAnimationFrame(this.loop.bind(this));
    }
    
    private loop(currentTime: number) {
        const elapsed = currentTime - this.lastTime;
        this.lastTime = currentTime;
        this.lag += elapsed;
        
        // Update game state at fixed rate
        while (this.lag >= this.msPerUpdate) {
            game.update(this.msPerUpdate / 1000);
            this.lag -= this.msPerUpdate;
        }
        
        // Render at variable rate
        game.render(this.lag / this.msPerUpdate);
        
        requestAnimationFrame(this.loop.bind(this));
    }
}
```

### WebSocket Client
```typescript
class GameWebSocket {
    private socket: WebSocket;
    private messageQueue: GameMessage[] = [];
    private reconnectAttempts: number = 0;
    private maxReconnectAttempts: number = 5;
    
    constructor(private url: string) {}
    
    connect() {
        this.socket = new WebSocket(this.url);
        
        this.socket.onopen = () => {
            this.reconnectAttempts = 0;
            this.flushMessageQueue();
            this.sendHandshake();
        };
        
        this.socket.onmessage = (event) => {
            this.handleMessage(event.data);
        };
        
        this.socket.onclose = () => {
            if (this.reconnectAttempts < this.maxReconnectAttempts) {
                setTimeout(() => this.connect(), 1000 * Math.pow(2, this.reconnectAttempts));
                this.reconnectAttempts++;
            }
        };
        
        this.socket.onerror = (error) => {
            console.error("WebSocket error:", error);
        };
    }
    
    send(message: GameMessage) {
        if (this.socket.readyState === WebSocket.OPEN) {
            this.socket.send(JSON.stringify(message));
        } else {
            this.messageQueue.push(message);
        }
    }
}
```

### Rendering System
```typescript
class CanvasRenderer {
    private canvas: HTMLCanvasElement;
    private ctx: CanvasRenderingContext2D;
    private camera: Camera;
    
    constructor(canvasId: string) {
        this.canvas = document.getElementById(canvasId) as HTMLCanvasElement;
        this.ctx = this.canvas.getContext('2d')!;
        this.camera = new Camera();
    }
    
    clear() {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    }
    
    drawSprite(sprite: Sprite, x: number, y: number) {
        const screenPos = this.camera.worldToScreen(x, y);
        this.ctx.drawImage(
            sprite.image,
            screenPos.x, screenPos.y,
            sprite.width, sprite.height
        );
    }
    
    drawTile(tile: Tile, x: number, y: number) {
        // Tile rendering logic
    }
}
```

## Performance Optimization

### Rendering Optimization
- Use offscreen canvases for complex elements
- Implement sprite batching
- Use requestAnimationFrame efficiently
- Minimize DOM manipulations

### Memory Management
- Reuse objects instead of creating new ones
- Implement object pooling
- Clean up unused resources
- Monitor memory usage

### Network Optimization
- Implement message batching
- Use binary protocols where possible
- Compress large messages
- Implement delta compression

## Error Handling

### Network Error Handling
```typescript
class NetworkErrorHandler {
    handleError(error: Error) {
        switch (error.type) {
            case 'connection':
                showNotification('Connection lost. Reconnecting...');
                break;
            case 'authentication':
                redirectToLogin();
                break;
            case 'rate_limit':
                showNotification('Too many requests. Please wait.');
                break;
            default:
                console.error('Network error:', error);
                showNotification('Network error occurred.');
        }
    }
}
```

### Rendering Error Handling
```typescript
class RenderingErrorHandler {
    handleWebGLError(error: WebGLContextEvent) {
        console.error('WebGL error:', error);
        // Fallback to Canvas 2D
        this.useCanvasFallback();
    }
    
    handleAssetLoadError(url: string) {
        console.error('Failed to load asset:', url);
        // Use placeholder asset
        this.usePlaceholder(url);
    }
}
```

## Testing Strategy

### Unit Tests
```typescript
// Example using Jest
describe('CombatSystem', () => {
    let combat: CombatSystem;
    
    beforeEach(() => {
        combat = new CombatSystem();
    });
    
    test('calculateDamage should return positive value', () => {
        const damage = combat.calculateDamage(attacker, defender, weapon);
        expect(damage).toBeGreaterThan(0);
        expect(damage).toBeLessThanOrEqual(weapon.maxDamage);
    });
});
```

### Integration Tests
```typescript
describe('Network Integration', () => {
    let client: GameClient;
    let mockServer: MockWebSocketServer;
    
    beforeAll(() => {
        mockServer = new MockWebSocketServer();
        client = new GameClient('ws://localhost:8081');
    });
    
    afterAll(() => {
        mockServer.close();
    });
    
    test('should handle handshake', async () => {
        await client.connect();
        const handshake = await mockServer.nextMessage();
        expect(handshake.type).toBe('handshake');
    });
});
```

## Build Pipeline

### Development Build
```json
{
  "scripts": {
    "dev": "webpack serve --mode development --hot",
    "watch": "tsc --watch",
    "lint": "eslint src/**/*.ts",
    "test": "jest"
  }
}
```

### Production Build
```json
{
  "scripts": {
    "build": "webpack --mode production",
    "build:ts": "tsc && tsc-alias",
    "build:assets": "copyfiles -u public/**/* ../assets",
    "build:all": "npm run build:ts && npm run build:assets",
    "analyze": "webpack-bundle-analyzer dist/stats.json"
  }
}
```

## Asset Management

### Asset Loading
```typescript
class AssetLoader {
    private loaded: Record<string, boolean> = {};
    private cache: Record<string, any> = {};
    
    load(manifest: AssetManifest): Promise<void> {
        return Promise.all(
            manifest.assets.map(asset => 
                this.loadSingleAsset(asset)
            )
        ).then(() => this.onAllLoaded());
    }
    
    private loadSingleAsset(asset: Asset): Promise<void> {
        return new Promise((resolve, reject) => {
            const loader = this.getLoaderForType(asset.type);
            loader.load(asset.url)
                .then(data => {
                    this.cache[asset.id] = data;
                    this.loaded[asset.id] = true;
                    resolve();
                })
                .catch(reject);
        });
    }
}
```

### Asset Optimization
- Compress images (PNG, JPEG optimization)
- Use sprite sheets for small images
- Minify JSON and other text assets
- Implement lazy loading
- Use appropriate formats (WebP, AVIF)

## Browser Compatibility

### Supported Browsers
- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+
- Mobile browsers with WebSocket support

### Polyfills
```typescript
// Polyfill for older browsers
import 'core-js/stable';
import 'regenerator-runtime/runtime';

// WebSocket polyfill if needed
if (!window.WebSocket) {
    window.WebSocket = require('websocket').w3cwebsocket;
}
```

## Debugging Tools

### Development Tools
- **Browser DevTools**: Chrome/Firefox developer tools
- **WebSocket Inspector**: Browser WebSocket debugging
- **Performance Profiler**: Browser performance tabs
- **Memory Analyzer**: Browser memory tools

### Debugging Techniques
```typescript
// Debug logging
development && console.log('Debug info:', variable);

// Performance marking
performance.mark('start-render');
// ... rendering code
performance.mark('end-render');
performance.measure('render-time', 'start-render', 'end-render');

// Error boundaries
class ErrorBoundary extends React.Component {
    componentDidCatch(error, info) {
        logErrorToService(error, info);
    }
}
```

## Future Enhancements

### Planned Improvements
1. **Performance**: WebGL acceleration, better batching
2. **Accessibility**: Better screen reader support
3. **Mobile**: Touch interface improvements
4. **Localization**: Multi-language support
5. **Modding**: Plugin architecture

### Technical Debt
- [ ] Improve error recovery
- [ ] Add more comprehensive tests
- [ ] Optimize asset loading
- [ ] Enhance mobile support

## Related Documentation

- **Architecture**: See `../../docs/architecture/README.md`
- **Protocol**: See `../../docs/protocol/README.md`
- **Gameplay**: See `../../docs/gameplay/README.md`
- **Operations**: See `../../docs/operations/README.md`