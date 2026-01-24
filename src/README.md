# Source Code Index

This directory contains all source code for the project organized by component.

## Structure

```
src/
├── README.md                # This file (code index)
├── server/                  # Rust server implementation
├── client/                  # TypeScript client source
├── assets/                  # Generated and static assets
├── db/                      # Database migrations and schema
└── ops/                     # Operational scripts and config
```

## Component Overview

### Server (Rust)
- **Location**: `src/server/`
- **Language**: Rust
- **Framework**: Actix Web + Tokio
- **Responsibilities**:
  - Game simulation and state management
  - WebSocket and HTTP endpoints
  - Database interactions
  - Authentication and security

### Client (TypeScript)
- **Location**: `src/client/`
- **Language**: TypeScript
- **Target**: Browser (compiled to JavaScript)
- **Responsibilities**:
  - User interface and rendering
  - Input handling and processing
  - Network communication
  - State management

### Assets
- **Location**: `src/assets/`
- **Content**:
  - Compiled JavaScript and CSS
  - Static images and sprites
  - Fonts and audio files
  - Generated asset manifests

### Database
- **Location**: `src/db/`
- **Content**:
  - SQL migration files
  - Schema documentation
  - Seed data
  - Query utilities

### Operations
- **Location**: `src/ops/`
- **Content**:
  - Dockerfiles and compose files
  - Configuration templates
  - Deployment scripts
  - Monitoring setup

## Development Workflow

### Adding New Features

1. **Identify Components**: Determine which components need changes
2. **Create Files**: Follow naming conventions and file size limits
3. **Update READMEs**: Add entries to relevant directory README files
4. **Implement**: Write code following project conventions
5. **Test**: Add appropriate tests
6. **Document**: Update documentation as needed

### File Organization Guidelines

- **File Size**: ≤ 200 lines per source file
- **Naming**: Use descriptive, concise names
- **Structure**: Prefer more files over longer files
- **Grouping**: Related functionality in same directory

### Component-Specific Guidelines

#### Server (Rust)
- Use `tokio` for async operations
- Use `actix-web` for HTTP/WebSocket
- Use `serde` for serialization
- Use `tracing` for logging
- Avoid global mutable state

#### Client (TypeScript)
- Target modern browsers (ES6+)
- Use Canvas 2D for rendering
- Minimize external dependencies
- Keep bundle size small
- Use TypeScript strict mode

#### Database
- Use PostgreSQL-specific features
- Write reproducible migrations
- Document schema changes
- Include rollback scripts

## Building the Project

### Development Build

```bash
# Build client assets
cd src/client
npm install
npm run dev

# Build Rust server
cd ../..
cargo build

# Start services
docker compose up --build
```

### Production Build

```bash
# Build optimized client
cd src/client
npm install
npm run build

# Build release server
cd ../..
cargo build --release

# Build Docker image
docker build -t game-server .
```

## Testing Strategy

### Unit Tests
- **Location**: Alongside source files
- **Framework**: Rust's built-in test framework
- **Coverage**: Critical logic and pure functions

### Integration Tests
- **Location**: `src/server/tests/`
- **Focus**: Component interactions
- **Examples**: Database + server integration

### End-to-End Tests
- **Location**: `src/tests/`
- **Focus**: Complete user flows
- **Examples**: Login → gameplay → logout

## Code Quality

### Formatting
- **Rust**: `cargo fmt`
- **TypeScript**: Prettier
- **Consistency**: Follow existing patterns

### Linting
- **Rust**: `cargo clippy`
- **TypeScript**: ESLint
- **Enforcement**: CI pipeline checks

### Documentation
- **Rust**: `cargo doc`
- **TypeScript**: JSDoc comments
- **Coverage**: Public APIs and complex logic

## Component Interfaces

### Server-Client Interface
- **Protocol**: WebSocket with custom binary/JSON
- **Endpoints**: Defined in protocol documentation
- **Authentication**: JWT tokens

### Server-Database Interface
- **Connection**: `sqlx` or `diesel`
- **Queries**: Parameterized queries
- **Transactions**: For multi-step operations

### Client-Asset Interface
- **Loading**: Dynamic imports
- **Caching**: Browser cache headers
- **Fallbacks**: Error handling

## Related Documentation

- **Architecture**: See `../docs/architecture/README.md`
- **Protocol**: See `../docs/protocol/README.md`
- **Gameplay**: See `../docs/gameplay/README.md`
- **Operations**: See `../docs/operations/README.md`