# Documentation and Source Structure

## Overview

This document describes the modular structure of the project's documentation and source code, organized according to the AGENTS.md specifications.

## Documentation Structure

### Root Documentation (`docs/`)

```
docs/
├── README.md                  # Documentation index
├── STRUCTURE.md               # This file
├── architecture/              # System architecture
│   ├── README.md
│   ├── server-authoritative/  # Server authority design
│   ├── networking/            # Network communication
│   ├── simulation/            # Game simulation
│   ├── persistence/           # Data persistence
│   └── frontend/              # Client architecture
├── protocol/                  # Network protocols
│   ├── README.md
│   ├── transport/             # Transport layer
│   ├── messages/              # Message formats
│   ├── serialization/         # Data serialization
│   └── security/              # Security protocols
├── gameplay/                  # Game mechanics
│   ├── README.md
│   ├── survival/              # Survival systems
│   ├── resources/             # Resource gathering
│   ├── crafting/              # Crafting system
│   ├── combat/                # Combat mechanics
│   ├── world/                 # World systems
│   └── progression/           # Progression systems
├── operations/                # Deployment and operations
│   ├── README.md
│   ├── deployment/            # Deployment procedures
│   ├── configuration/         # Configuration management
│   ├── database/              # Database operations
│   ├── monitoring/            # Monitoring setup
│   └── scaling/               # Scaling strategies
└── decisions/                 # Architectural decisions
    ├── README.md
    └── adr-XXXX-template.md   # ADR template
```

## Source Code Structure

### Root Source (`src/`)

```
src/
├── README.md                  # Source code index
├── server/                    # Rust server
│   ├── README.md
│   ├── config/                # Configuration
│   ├── network/               # Network handlers
│   ├── simulation/            # Game simulation
│   ├── world/                 # World management
│   ├── systems/               # Game systems
│   ├── db/                    # Database interface
│   └── protocol/              # Protocol implementation
├── client/                    # TypeScript client
│   ├── README.md
│   ├── game/                  # Game logic
│   ├── network/               # Network communication
│   ├── ui/                    # User interface
│   ├── rendering/             # Rendering system
│   ├── input/                 # Input handling
│   ├── state/                 # State management
│   └── assets/                # Asset management
├── assets/                    # Compiled assets
│   ├── README.md
│   ├── js/                    # JavaScript bundles
│   ├── css/                   # CSS styles
│   ├── images/                # Image assets
│   ├── sounds/                # Audio assets
│   ├── fonts/                 # Font files
│   └── manifests/             # Asset manifests
├── db/                        # Database
│   ├── README.md
│   ├── migrations/            # Migration scripts
│   ├── seeds/                 # Seed data
│   ├── schema/                # Schema documentation
│   ├── queries/               # Common queries
│   └── scripts/               # Database scripts
└── ops/                       # Operations
    ├── README.md
    ├── docker/                # Docker configuration
    ├── compose/               # Docker Compose
    ├── scripts/               # Operational scripts
    ├── config/                # Configuration
    ├── monitoring/            # Monitoring
    └── deployment/            # Deployment
```

## Structure Principles

### 1. Modular Organization
- Each directory has exactly one README.md as table of contents
- Related functionality grouped together
- Clear separation of concerns

### 2. Size Constraints
- Documentation files: ≤ 300 lines
- Source code files: ≤ 200 lines
- Deep directory trees preferred over long files

### 3. Cross-Referencing
- README files link to related documentation
- Clear navigation between components
- Contextual references

### 4. Progressive Detail
- High-level overview at top
- Increasing detail at lower levels
- Implementation specifics at leaves

## Navigation Guide

### Finding Information

1. **Start at root**: `README.md` files provide overview
2. **Follow structure**: Navigate through directory hierarchy
3. **Use cross-references**: Follow links to related content
4. **Check structure**: Consult this file for orientation

### Adding New Content

1. **Identify location**: Find appropriate directory
2. **Create directory**: If needed for new component
3. **Add README**: Table of contents for new directory
4. **Add content**: Specific documentation files
5. **Update references**: Link from parent directories

## Maintenance

### Keeping Structure Current
- Update README files when adding new components
- Remove unused directories and files completely
- Keep cross-references accurate
- Review structure periodically

### Documentation Standards
- Clear, concise language
- Code examples where helpful
- Consistent formatting
- Accurate cross-references

## Related Documentation

- **Architecture**: `architecture/README.md`
- **Protocol**: `protocol/README.md`
- **Gameplay**: `gameplay/README.md`
- **Operations**: `operations/README.md`