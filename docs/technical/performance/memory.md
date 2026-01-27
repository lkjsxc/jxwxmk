# Memory Optimization Strategy

Goal: keep runtime RSS bounded with chunk streaming and strict caps.

## Techniques

### 1. Dependency Control
- Prefer small dependency surface.
- Avoid extra runtime services beyond PostgreSQL.

### 2. Build Optimizations
- Release builds with LTO and stripping.

### 3. Runtime Configuration
- Single worker to cap memory.
- Bounded queues for input, generation, and network.

### 4. World Data
- Chunk-level storage with LRU eviction.
- Compact serialization for frozen chunks.

### 5. Asset Strategy
- Compile frontend assets once and embed using `rust-embed`.
