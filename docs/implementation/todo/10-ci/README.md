# 10 — CI (GitHub Actions)

Goal: CI builds the runtime container on every push/PR.

References:
- `docs/technical/deployment/ci.md`

## A) Build job

- [ ] Add `.github/workflows/ci.yml` that triggers on:
  - `push`
  - `pull_request`
- [ ] Job builds the image:
  - `docker build -f src/runtime/Dockerfile -t jxwxmk:ci .`
- [ ] Job fails on any build error.

## B) Optional smoke test

- [ ] Optionally run the container and hit `/health` (per `docs/technical/deployment/ci.md`).

## Done when

- [ ] CI builds successfully on a clean runner.
