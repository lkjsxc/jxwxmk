-- Initial database schema for the survival game

-- Players table
CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_login TIMESTAMP WITH TIME ZONE
);

-- Player sessions table
CREATE TABLE IF NOT EXISTS player_sessions (
    id UUID PRIMARY KEY,
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    token VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Player inventory table
CREATE TABLE IF NOT EXISTS player_inventory (
    id UUID PRIMARY KEY,
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    item_type VARCHAR(50) NOT NULL,
    item_id VARCHAR(50),
    quantity INTEGER NOT NULL DEFAULT 1,
    slot INTEGER,
    equipped BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- World resources table
CREATE TABLE IF NOT EXISTS world_resources (
    id UUID PRIMARY KEY,
    resource_type VARCHAR(50) NOT NULL,
    position_x FLOAT NOT NULL,
    position_y FLOAT NOT NULL,
    quantity FLOAT NOT NULL,
    max_quantity FLOAT NOT NULL,
    respawn_time TIMESTAMP WITH TIME ZONE,
    biome VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Player progression table
CREATE TABLE IF NOT EXISTS player_progression (
    id UUID PRIMARY KEY,
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    experience INTEGER NOT NULL DEFAULT 0,
    level INTEGER NOT NULL DEFAULT 1,
    skill_points INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Player skills table
CREATE TABLE IF NOT EXISTS player_skills (
    id UUID PRIMARY KEY,
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    skill_name VARCHAR(50) NOT NULL,
    skill_level INTEGER NOT NULL DEFAULT 1,
    skill_experience INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(player_id, skill_name)
);

-- Crafting recipes table
CREATE TABLE IF NOT EXISTS crafting_recipes (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    tier INTEGER NOT NULL DEFAULT 1,
    crafting_time FLOAT NOT NULL DEFAULT 1.0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Recipe requirements table
CREATE TABLE IF NOT EXISTS recipe_requirements (
    id UUID PRIMARY KEY,
    recipe_id UUID NOT NULL REFERENCES crafting_recipes(id) ON DELETE CASCADE,
    item_type VARCHAR(50) NOT NULL,
    quantity INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Recipe results table
CREATE TABLE IF NOT EXISTS recipe_results (
    id UUID PRIMARY KEY,
    recipe_id UUID NOT NULL REFERENCES crafting_recipes(id) ON DELETE CASCADE,
    item_type VARCHAR(50) NOT NULL,
    quantity INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Game events log table
CREATE TABLE IF NOT EXISTS game_events (
    id UUID PRIMARY KEY,
    event_type VARCHAR(50) NOT NULL,
    player_id UUID REFERENCES players(id) ON DELETE SET NULL,
    entity_id VARCHAR(50),
    position_x FLOAT,
    position_y FLOAT,
    details JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX idx_player_inventory_player ON player_inventory(player_id);
CREATE INDEX idx_world_resources_position ON world_resources(position_x, position_y);
CREATE INDEX idx_player_sessions_token ON player_sessions(token);
CREATE INDEX idx_player_sessions_expires ON player_sessions(expires_at);
CREATE INDEX idx_game_events_player ON game_events(player_id);
CREATE INDEX idx_game_events_type ON game_events(event_type);
CREATE INDEX idx_game_events_time ON game_events(created_at);