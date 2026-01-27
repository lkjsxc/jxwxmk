# Memory Optimization Strategy

Goal: keep runtime RSS around ~20MB (single container, single server process, embedded assets).

## Techniques

### 1. Dependency Control
- Prefer small dependency surface (Actix Web + RustEmbed + Serde).
- Avoid extra runtime services in-process beyond PostgreSQL.

### 2. Build Optimizations
- Favor release builds with LTO and stripping.
- Keep the Rust binary small by avoiding heavy optional features.

### 3. Runtime Configuration
- Run Actix with a single worker (`workers(1)`) to cap memory.
- Avoid background tasks that allocate large buffers.
- Keep world snapshot serialization bounded by world size limits.

### 4. Data Structures
- Prefer small, explicit structs over nested dynamic maps.
- Avoid per-tick allocations; reuse vectors where possible.

### 5. Static Asset Strategy
- Compile frontend assets once and embed using `rust-embed`.
- Serve from memory to avoid file I/O.
