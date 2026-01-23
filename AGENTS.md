# AGENTS.md

This file contains guidelines for agentic coding agents working in this repository.

## Project Overview
Game like starve.io - a multiplayer survival game with rich content and simple graphics.
Tech stack: Rust, tokio, Actix Web, PostgreSQL, TypeScript, Docker Compose.

## Directory Structure
- Project root: README.md, LICENSE, AGENTS.md, docs/, src/, and hidden files/dirs only
- Each directory has exactly one README.md serving as table of contents
- Documentation files: max 300 lines each
- Source code files: max 200 lines each
- Use recursive directory structure (tree format)

## Build Commands

### Rust Backend
```bash
# Build the project
cargo build

# Run in development mode
cargo run

# Run tests
cargo test

# Run single test (replace test_name)
cargo test test_name

# Run clippy linter
cargo clippy -- -D warnings

# Check formatting
cargo fmt --check

# Format code
cargo fmt
```

### TypeScript Frontend
```bash
# Install dependencies
npm install

# Build for development
npm run dev

# Build for production
npm run build

# Run tests
npm test

# Run single test (replace test_name)
npm test -- --testNamePattern="test_name"

# Run linter
npm run lint

# Run type checker
npm run typecheck

# Format code
npm run format
```

### Docker Compose
```bash
# Start all services
docker-compose up

# Start in background
docker-compose up -d

# Stop services
docker-compose down

# Rebuild and start
docker-compose up --build

# View logs
docker-compose logs -f
```

## Code Style Guidelines

### Rust Code Style
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Prefer `async/await` with tokio
- Use `Result<T, E>` for error handling
- Struct names: PascalCase
- Function/variable names: snake_case
- Constants: SCREAMING_SNAKE_CASE
- Use `#[derive(Debug)]` for public structs
- Prefer `#[tokio::main]` for async main functions

### TypeScript Code Style
- Use Prettier for formatting
- Use ESLint for linting
- Use TypeScript strict mode
- Interface/Type names: PascalCase
- Function/variable names: camelCase
- Constants: UPPER_SNAKE_CASE
- Use `async/await` for asynchronous operations
- Prefer explicit return types for functions

### Import Organization
#### Rust
```rust
// Standard library
use std::collections::HashMap;

// External crates
use tokio::net::TcpListener;
use actix_web::{web, App, HttpServer};

// Internal modules
use crate::models::User;
use crate::handlers::auth;
```

#### TypeScript
```typescript
// Node.js built-ins
import { readFileSync } from 'fs';

// External libraries
import express from 'express';
import { WebSocket } from 'ws';

// Internal modules
import { Game } from '../models/Game';
import { authMiddleware } from '../middleware/auth';
```

## Error Handling

### Rust
- Use `Result<T, E>` for fallible functions
- Create custom error types using `thiserror`
- Use `?` operator for error propagation
- Log errors appropriately using `tracing`

### TypeScript
- Use try-catch blocks for async operations
- Create custom error classes
- Return early on errors
- Use proper HTTP status codes

## Database Guidelines
- Use PostgreSQL with proper migrations
- All database operations should be async
- Use connection pooling
- Write queries in a consistent style
- Use prepared statements for security

## Testing Guidelines
- Write unit tests for all business logic
- Write integration tests for API endpoints
- Test error cases and edge cases
- Use descriptive test names
- Keep tests focused and independent

## Git Guidelines
- Commit frequently with descriptive messages
- Use conventional commit format
- Never commit secrets or API keys
- Create PRs for significant changes

## Documentation Standards
- Each directory has one README.md (max 300 lines)
- Document public APIs thoroughly
- Use inline comments sparingly (only when necessary)
- Keep documentation up to date with code changes

## Performance Guidelines
- Optimize database queries
- Use caching where appropriate
- Monitor memory usage
- Profile code before optimizing
- Consider async operations for I/O bound tasks

## Security Guidelines
- Validate all inputs
- Use parameterized queries
- Implement proper authentication
- Use HTTPS in production
- Never expose sensitive data

## Development Workflow
1. Create feature branch
2. Implement functionality
3. Write tests
4. Run linter and formatter
5. Run test suite
6. Commit changes
7. Create PR if needed

## File Size Limits
- Source files: max 200 lines
- Documentation files: max 300 lines
- Break large files into smaller modules

## Naming Conventions
- Files: kebab-case for most files
- Directories: kebab-case
- Database tables: snake_case
- API endpoints: kebab-case
- Environment variables: UPPER_SNAKE_CASE