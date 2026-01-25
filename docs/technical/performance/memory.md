# Memory Optimization Strategy

Goal: Maintain runtime memory usage (RSS) around 20MB.

## Techniques

### 1. Dependency Pruning
-   **Feature Flags**: Disable unused features in `actix-web`, `tokio`, and `sqlx`.
-   **Allocator**: Use the system allocator (default in Rust) to avoid overhead, or switch to `jemalloc` if fragmentation occurs (though system is usually lighter for small apps).

### 2. Compilation Optimization
-   `opt-level = "z"`: Optimize for binary size (often correlates with lower instruction cache pressure).
-   `lto = true`: Link Time Optimization removes dead code.
-   `codegen-units = 1`: Maximizes optimization at the cost of compile time.
-   `strip = true`: Removes symbols to reduce binary size.

### 3. Runtime Configuration
-   **Worker Threads**: Limit Tokio/Actix worker threads. Default is equal to CPU cores. For a low-memory target, we constrain this to `1` or `2`.
-   **Connection Pools**: If a DB is used, limit `max_connections` to 1.

### 4. Data Structures
-   **Zero-Copy**: Use `Cow<str>` or references where possible.
-   **String Interning**: If many entities share names, intern them (future work).
