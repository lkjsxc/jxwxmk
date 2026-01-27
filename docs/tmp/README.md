# Templates (Agent Prompts)

This directory contains **agent-facing prompt templates** used to run work in this repository (changes, docs-only edits, and full `src/` reconstruction). These are process tools, not gameplay specs.

## Contents

- [Change Template Prompt (Expanded)](content-change.md): base prompt for any repo change.
- [Docs-Only Update Prompt](docs-update.md): docs-only overlay for the base change template.
- [Source Tree Reconstruction](src-recreate.md): prompt for rebuilding `src/` from documentation.

## Usage

- Prefer the smallest prompt that matches the work:
  - docs-only → `docs-update.md`
  - any code/system change → `content-change.md`
  - full rebuild → `src-recreate.md`
- Follow directory-local `README.md` TOCs instead of hard-coding long file lists in prompts.
