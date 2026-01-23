#!/bin/bash

echo "Testing project structure..."

# Test directory structure
echo "Checking directories..."
if [ -d "src/backend" ] && [ -d "src/frontend" ] && [ -d "docs" ]; then
    echo "✓ Directory structure is correct"
else
    echo "✗ Directory structure is incomplete"
    exit 1
fi

# Test backend files
echo "Checking backend files..."
if [ -f "src/backend/Cargo.toml" ] && [ -f "src/backend/src/main.rs" ]; then
    echo "✓ Backend files exist"
else
    echo "✗ Backend files missing"
    exit 1
fi

# Test frontend files
echo "Checking frontend files..."
if [ -f "src/frontend/package.json" ] && [ -f "src/frontend/src/main.ts" ]; then
    echo "✓ Frontend files exist"
else
    echo "✗ Frontend files missing"
    exit 1
fi

# Test documentation
echo "Checking documentation..."
if [ -f "docs/README.md" ] && [ -f "docs/architecture/README.md" ]; then
    echo "✓ Documentation files exist"
else
    echo "✗ Documentation files missing"
    exit 1
fi

# Test Docker configuration
echo "Checking Docker files..."
if [ -f "docker-compose.yml" ] && [ -f "Dockerfile.backend" ] && [ -f "Dockerfile.frontend" ]; then
    echo "✓ Docker files exist"
else
    echo "✗ Docker files missing"
    exit 1
fi

echo ""
echo "Project structure test completed successfully!"
echo ""
echo "Project summary:"
echo "- Backend: Rust with Actix Web"
echo "- Frontend: TypeScript with Vite"
echo "- Database: PostgreSQL"
echo "- Containerization: Docker Compose"
echo "- Documentation: Complete structure"

echo ""
echo "Next steps:"
echo "1. Run 'docker-compose up -d' to start services"
echo "2. Run 'cargo run' in src/backend to start backend"
echo "3. Run 'npm run dev' in src/frontend to start frontend"