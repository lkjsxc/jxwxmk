#!/bin/bash
set -e

echo "=== JXWXMK Build Script ==="

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo -e "${RED}Error: Docker is not installed${NC}"
    exit 1
fi

# Check if Docker daemon is running
if ! docker info &> /dev/null; then
    echo -e "${RED}Error: Docker daemon is not running${NC}"
    exit 1
fi

# Build options
BUILD_TYPE="${1:-standard}"
IMAGE_TAG="jxwxmk:latest"

case "$BUILD_TYPE" in
    quick)
        echo -e "${YELLOW}Using quick build (cargo-chef)...${NC}"
        DOCKERFILE="src/runtime/Dockerfile.quick"
        ;;
    standard|*)
        echo -e "${YELLOW}Using standard build...${NC}"
        DOCKERFILE="src/runtime/Dockerfile"
        ;;
esac

# Build
echo "Building Docker image..."
docker build -f "$DOCKERFILE" -t "$IMAGE_TAG" .

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build successful!${NC}"
    echo ""
    echo "To run the container:"
    echo "  make run          # or: docker run --rm -p 8080:8080 -v jxwxmk_pgdata:/var/lib/postgresql/data -v \$(PWD)/config:/app/config:ro $IMAGE_TAG"
    echo ""
    echo "To test endpoints:"
    echo "  make test         # or: curl http://localhost:8080/health"
else
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi
