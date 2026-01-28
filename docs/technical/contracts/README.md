# System Contracts

This section defines the **cross-cutting invariants** that make the system cohesive and debuggable.

Treat these as “build-breaking” constraints: if code contradicts a contract, code is wrong (or a decision record is required).

## Contents

- [Authority + Ownership](authority.md)
- [Tick + Time](tick.md)
- [World Space + IDs](world_space.md)
- [Protocol Contract (versioning, errors, limits)](protocol.md)
- [Config Contract (schemas, defaults, validation)](config.md)
- [Persistence Contract (migrations, checkpoints)](persistence.md)
