-- Create accounts table
CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create players table
CREATE TABLE players (
    id VARCHAR(100) PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id),
    position_x REAL NOT NULL DEFAULT 0.0,
    position_y REAL NOT NULL DEFAULT 0.0,
    health REAL NOT NULL DEFAULT 100.0,
    hunger REAL NOT NULL DEFAULT 100.0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create inventory table (authoritative)
CREATE TABLE inventory (
    id SERIAL PRIMARY KEY,
    account_id INTEGER REFERENCES accounts(id) ON DELETE CASCADE,
    item_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    UNIQUE(account_id, item_id)
);

-- Create world_facts table (minimal long-lived data)
CREATE TABLE world_facts (
    id SERIAL PRIMARY KEY,
    node_type VARCHAR(50) NOT NULL,
    position_x REAL NOT NULL,
    position_y REAL NOT NULL,
    respawn_time TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);