# Agent Guidelines: kkmypk

This repository is optimized for LLM-based development. Adhere to these rules strictly.

## 1. Build, Lint, and Test Commands

### Backend (Rust)
- Build: `cargo build`
- Run: `cargo run`
- Test All: `cargo test`
- Test Single: `cargo test <test_name>`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt`

### Frontend (TypeScript)
- Install: `npm install`
- Build: `npm run build`
- Test All: `npm test`
- Test Single: `npx jest <file_path>`
- Lint: `npm run lint`
- Format: `npm run format`

### Environment
- Start Infrastructure: `docker-compose up -d`
- Stop Infrastructure: `docker-compose down`

## 2. Code Style and Architecture

### General Principles
- **LLM-First:** Prioritize machine readability and structured logic over human-friendly prose.
- **File Limits:** 
    - Source code files MUST NOT exceed **200 lines**.
    - Documentation files MUST NOT exceed **300 lines**.
- **Directory Structure:** Use deep recursive tree structures. Every directory must contain exactly one `README.md` serving as a Table of Contents.
- **No Backward Compatibility:** Break things whenever necessary to achieve the best current design.

### Rust Style (Backend)
- **Frameworks:** Actix Web for API, Tokio for async runtime, SQLx for PostgreSQL.
- **Naming:** `snake_case` for functions and variables, `PascalCase` for types and traits.
- **Error Handling:** Use `anyhow::Result` for application logic and `thiserror` for library-level errors. Never use `unwrap()` or `expect()` in production code.
- **Imports:** Grouped by: 1. Standard library, 2. Third-party crates, 3. Local modules.
- **Types:** Explicit type annotations for all public APIs and complex logic.

### TypeScript Style (Frontend)
- **Naming:** `camelCase` for variables and functions, `PascalCase` for components and types.
- **Strictness:** `strict: true` in `tsconfig.json`. Avoid `any`.
- **Imports:** Use absolute paths or aliases (e.g., `@/components/...`).
- **Error Handling:** Use try-catch blocks with typed error checking.

## 3. Documentation
- Each directory must have a `README.md`.
- Documentation should be concise and technically dense.
- Update documentation immediately after or during source code changes.

## 4. Git Workflow
- Commit frequently with concise, descriptive messages.
- Always run linting and tests before committing.

## 5. Game Context (Starve.io Clone)
- Focus on performance and real-time synchronization.
- Backend handles physics, crafting, and state.
- Frontend handles rendering (Canvas/WebGL) and user input.
