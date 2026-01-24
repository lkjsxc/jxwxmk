# Documentation Index

This directory contains all project documentation organized by domain.

## Structure

```
docs/
├── README.md                # This file (documentation index)
├── architecture/            # System architecture and design
├── protocol/               # Network protocol specifications
├── gameplay/               # Game mechanics and systems
├── operations/             # Deployment and operational guides
└── decisions/              # Architectural Decision Records (ADRs)
```

## Documentation Areas

### Architecture
- **Location**: `docs/architecture/`
- **Purpose**: High-level system design, component interactions, and architectural principles
- **Audience**: Developers, architects, and maintainers
- **Key Topics**:
  - Server-client architecture
  - Simulation loop design
  - State management patterns
  - Performance considerations

### Protocol
- **Location**: `docs/protocol/`
- **Purpose**: Network communication specifications and message formats
- **Audience**: Network engineers, client/server developers
- **Key Topics**:
  - Message structure and versioning
  - WebSocket communication patterns
  - Serialization formats
  - Error handling and recovery

### Gameplay
- **Location**: `docs/gameplay/`
- **Purpose**: Game mechanics, systems, and content design
- **Audience**: Game designers, content creators, gameplay programmers
- **Key Topics**:
  - Core survival mechanics (hunger, health, etc.)
  - Crafting and progression systems
  - Combat and interaction rules
  - World generation and resource distribution

### Operations
- **Location**: `docs/operations/`
- **Purpose**: Deployment, configuration, and operational procedures
- **Audience**: DevOps engineers, system administrators
- **Key Topics**:
  - Docker Compose setup and configuration
  - Environment variables and settings
  - Database migrations and management
  - Monitoring and logging
  - Scaling considerations

### Decisions (ADRs)
- **Location**: `docs/decisions/`
- **Purpose**: Architectural Decision Records documenting key design choices
- **Audience**: All team members and future maintainers
- **Key Topics**:
  - Major technology choices
  - Design trade-offs and rationale
  - Deprecated approaches and lessons learned
  - Future considerations and alternatives

## Documentation Standards

### File Organization
- Each directory must contain exactly one `README.md` as its table of contents
- Additional documentation files should be organized in logical subdirectories
- Keep documentation files focused and concise (≤ 300 lines per file)

### Content Guidelines
- Use clear, concise language
- Include code examples where helpful
- Document assumptions and constraints
- Link to related documentation and source code
- Keep documentation up-to-date with code changes

### Versioning
- Documentation should reflect the current state of the system
- Major changes should be documented in ADRs
- Deprecated documentation should be removed, not archived in-place

## Getting Started

1. **New to the project?** Start with:
   - `docs/architecture/README.md` - Understand the system architecture
   - `docs/protocol/README.md` - Learn about network communication
   - `docs/gameplay/README.md` - Explore game mechanics

2. **Setting up the environment?** See:
   - `docs/operations/README.md` - Deployment and configuration
   - `../README.md` - Quickstart guide

3. **Making architectural decisions?** Consult and contribute to:
   - `docs/decisions/README.md` - ADR process and existing decisions

## Maintenance

- Update documentation whenever code behavior or structure changes
- Remove outdated or irrelevant documentation
- Keep the table of contents in each README.md current
- Review documentation as part of the code review process