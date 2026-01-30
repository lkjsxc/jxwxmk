# 10 — CI (GitHub Actions)

Goal: CI builds the runtime container on every push/PR.

References:
- `docs/technical/deployment/ci.md`

## A) Build job

- [x] Add `.github/workflows/ci.yml` that triggers on:
  - `push`
  - `pull_request`
- [x] Job builds the image:
  - `docker build -f src/runtime/Dockerfile -t jxwxmk:ci .`
- [x] Job fails on any build error.

## B) Optional smoke test

- [x] Optionally run the container and hit `/health` and `/metrics` (per `docs/technical/deployment/ci.md`).

## Done when

- [x] CI builds successfully on a clean runner.
