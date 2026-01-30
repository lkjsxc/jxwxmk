# JXWXMK Makefile

.PHONY: build build-quick run stop test clean logs shell

# Default target
all: build

# Build the Docker image
build:
	docker build -f src/runtime/Dockerfile -t jxwxmk:latest .

# Quick build with cargo-chef (faster for rebuilds)
build-quick:
	docker build -f src/runtime/Dockerfile.quick -t jxwxmk:latest .

# Run the container
run:
	docker run --rm -d \
		--name jxwxmk \
		-p 8080:8080 \
		-v jxwxmk_pgdata:/var/lib/postgresql/data \
		-v $(PWD)/config:/app/config:ro \
		jxwxmk:latest

# Run with docker-compose
up:
	docker compose -f src/runtime/compose/docker-compose.yml up -d

# Stop the container
stop:
	docker stop jxwxmk 2>/dev/null || true
	docker rm jxwxmk 2>/dev/null || true

# Stop docker-compose
down:
	docker compose -f src/runtime/compose/docker-compose.yml down

# View logs
logs:
	docker logs -f jxwxmk

# Test endpoints
test:
	@echo "Testing health endpoint..."
	@curl -s http://localhost:8080/health && echo ""
	@echo "Testing metrics endpoint..."
	@curl -s http://localhost:8080/metrics | head -5

# Clean up
clean:
	docker rmi jxwxmk:latest 2>/dev/null || true
	docker volume rm jxwxmk_pgdata 2>/dev/null || true

# Shell into running container
shell:
	docker exec -it jxwxmk /bin/bash

# Build and run in one command
dev: build run
	@echo "Server running at http://localhost:8080"
	@echo "Run 'make logs' to view logs"
	@echo "Run 'make test' to test endpoints"

# Quick dev build (no TypeScript rebuild)
dev-quick:
	docker build -f src/runtime/Dockerfile --target rust-builder -t jxwxmk:builder .
	docker build -f src/runtime/Dockerfile -t jxwxmk:latest .
	$(MAKE) run
