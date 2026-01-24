-- Initial seed data

-- Sample accounts
INSERT INTO accounts (username, password_hash) VALUES
('player1', '$2b$12$examplehash1'),  -- Use bcrypt hashes in real impl
('player2', '$2b$12$examplehash2');

-- Sample inventory
INSERT INTO inventory (account_id, item_id, quantity) VALUES
(1, 0, 10),  -- Player1: 10 wood
(1, 1, 5),   -- Player1: 5 stone
(2, 0, 20);  -- Player2: 20 wood

-- Sample world facts (resource nodes)
INSERT INTO world_facts (node_type, position_x, position_y) VALUES
('tree', 100.0, 200.0),
('rock', 150.0, 250.0);