#!/bin/bash

echo "=== Verifying Project Structure ==="
echo

# Check root structure
echo "1. Checking root directory structure..."
required_root_files=("README.md" "LICENSE" "AGENTS.md" "docs" "src" "docker-compose.yml" ".env.example" ".gitignore")
missing_root=()

for file in "${required_root_files[@]}"; do
    if [ ! -e "$file" ]; then
        missing_root+=("$file")
    fi
done

if [ ${#missing_root[@]} -eq 0 ]; then
    echo "✓ All root files present"
else
    echo "✗ Missing root files: ${missing_root[*]}"
fi

echo

# Check server structure
echo "2. Checking server structure..."
required_server_files=("src/server/Cargo.toml" "src/server/src/main.rs" "src/server/src/config.rs" 
                      "src/server/src/simulation.rs" "src/server/src/network.rs" 
                      "src/server/src/world.rs" "src/server/src/systems.rs" 
                      "src/server/src/db.rs" "src/server/src/protocol.rs")
missing_server=()

for file in "${required_server_files[@]}"; do
    if [ ! -e "$file" ]; then
        missing_server+=("$file")
    fi
done

if [ ${#missing_server[@]} -eq 0 ]; then
    echo "✓ All server files present"
else
    echo "✗ Missing server files: ${missing_server[*]}"
fi

echo

# Check client structure
echo "3. Checking client structure..."
required_client_files=("src/client/package.json" "src/client/webpack.config.js" "src/client/tsconfig.json"
                      "src/client/src/index.ts" "src/client/src/index.html"
                      "src/client/src/game/GameClient.ts" "src/client/src/game/GameState.ts"
                      "src/client/src/game/Player.ts" "src/client/src/game/World.ts"
                      "src/client/src/network/NetworkManager.ts"
                      "src/client/src/input/InputManager.ts" "src/client/src/input/InputState.ts"
                      "src/client/src/rendering/Renderer.ts"
                      "src/client/src/ui/UIManager.ts")
missing_client=()

for file in "${required_client_files[@]}"; do
    if [ ! -e "$file" ]; then
        missing_client+=("$file")
    fi
done

if [ ${#missing_client[@]} -eq 0 ]; then
    echo "✓ All client files present"
else
    echo "✗ Missing client files: ${missing_client[*]}"
fi

echo

# Check database structure
echo "4. Checking database structure..."
required_db_files=("src/db/migrations/0001_initial_schema.sql")
missing_db=()

for file in "${required_db_files[@]}"; do
    if [ ! -e "$file" ]; then
        missing_db+=("$file")
    fi
done

if [ ${#missing_db[@]} -eq 0 ]; then
    echo "✓ All database files present"
else
    echo "✗ Missing database files: ${missing_db[*]}"
fi

echo

# Check configuration files
echo "5. Checking configuration files..."
required_config_files=("config/default.toml" "config/local.toml" "Dockerfile")
missing_config=()

for file in "${required_config_files[@]}"; do
    if [ ! -e "$file" ]; then
        missing_config+=("$file")
    fi
done

if [ ${#missing_config[@]} -eq 0 ]; then
    echo "✓ All configuration files present"
else
    echo "✗ Missing configuration files: ${missing_config[*]}"
fi

echo

# File count summary
echo "6. File count summary..."
echo "   Server files: $(find src/server -name '*.rs' | wc -l)"
echo "   Client files: $(find src/client -name '*.ts' -o -name '*.js' | wc -l)"
echo "   Database files: $(find src/db -name '*.sql' | wc -l)"
echo "   Config files: $(find config -name '*.toml' | wc -l)"

echo
echo "=== Structure Verification Complete ==="

# Exit with error if any files are missing
total_missing=$(( ${#missing_root[@]} + ${#missing_server[@]} + ${#missing_client[@]} + ${#missing_db[@]} + ${#missing_config[@]} ))
if [ $total_missing -gt 0 ]; then
    echo "❌ Found $total_missing missing files"
    exit 1
else
    echo "✅ All files present and accounted for!"
    exit 0
fi