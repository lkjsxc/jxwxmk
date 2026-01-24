# Architecture

This section details the technical implementation of the game.

## Overview

The application is a monolithic Rust server that handles:
1.  **HTTP Serving**: Static assets (HTML, JS, CSS) for the game client.
2.  **Game Logic**: Authoritative server loop running on Tokio.
3.  **Real-time Comms**: WebSockets for player state synchronization.
4.  **Persistence**: PostgreSQL connection for user accounts and leaderboards.

## Contents

- [Tech Stack](tech_stack.md)
- [Server Architecture](server.md)
- [Client Architecture](client.md)
- [Database Schema](database.md)
