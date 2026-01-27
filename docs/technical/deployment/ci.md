# CI (GitHub Actions)

The CI pipeline must build the single runtime container on every push and pull request.

## Requirements

- Trigger on `push` and `pull_request` (main + release branches).
- Build the container from the repository Dockerfile.
- Fail the job on any build error.
- No publishing by default (publishing can be added later on tags).
- Optional: run the container and hit `/health`.

## Suggested Job Outline

1. Checkout repository.
2. Set up Docker Buildx (and QEMU if multi-arch is required).
3. Run `docker build -f src/runtime/Dockerfile -t jxwxmk:ci .`.
4. (Optional) Run the container and validate `/health`.
