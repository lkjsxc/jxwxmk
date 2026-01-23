-- Create players table
CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create characters table
CREATE TABLE characters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    name VARCHAR(50) NOT NULL,
    x FLOAT NOT NULL DEFAULT 0,
    y FLOAT NOT NULL DEFAULT 0,
    health INTEGER NOT NULL DEFAULT 100,
    hunger INTEGER NOT NULL DEFAULT 100,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create items table
CREATE TABLE items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    item_type VARCHAR(50) NOT NULL,
    stack_size INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create inventory table
CREATE TABLE inventory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    item_id UUID NOT NULL REFERENCES items(id),
    quantity INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(character_id, item_id)
);

-- Create worlds table
CREATE TABLE worlds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    seed BIGINT NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create game_sessions table
CREATE TABLE game_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    character_id UUID NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    world_id UUID NOT NULL REFERENCES worlds(id),
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_activity TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Insert basic items
INSERT INTO items (name, item_type, stack_size) VALUES
('Wood', 'resource', 99),
('Stone', 'resource', 99),
('Food', 'consumable', 20),
('Wooden Axe', 'tool', 1),
('Wooden Pickaxe', 'tool', 1),
('Stone Axe', 'tool', 1),
('Stone Pickaxe', 'tool', 1);

-- Create indexes for performance
CREATE INDEX idx_characters_player_id ON characters(player_id);
CREATE INDEX idx_inventory_character_id ON inventory(character_id);
CREATE INDEX idx_game_sessions_character_id ON game_sessions(character_id);
CREATE INDEX idx_game_sessions_world_id ON game_sessions(world_id);