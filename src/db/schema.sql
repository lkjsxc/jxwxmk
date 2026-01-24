-- Full database schema for starve.io game

-- Accounts: user sessions and auth
CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Inventory: authoritative player items
CREATE TABLE inventory (
    id SERIAL PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id) ON DELETE CASCADE,
    item_id INTEGER NOT NULL,  -- 0: wood, 1: stone, etc.
    quantity INTEGER NOT NULL DEFAULT 0,
    UNIQUE(account_id, item_id)
);

-- World facts: persistent resource nodes (minimal)
CREATE TABLE world_facts (
    id SERIAL PRIMARY KEY,
    node_type VARCHAR(50) NOT NULL,
    position_x REAL NOT NULL,
    position_y REAL NOT NULL,
    respawn_time TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);