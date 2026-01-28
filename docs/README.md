# Documentation Root

This documentation set is the source of truth for the project. It must be sufficient to re-implement the game without the code.

## Start Here

- [Policy + Operating Contract](policy/INSTRUCT.md)
- [Scope boundary for initial reconstruction](decisions/0001-reconstruction-scope.md)
- [Reconstruction acceptance criteria (“definition of done”)](implementation/reconstruction_acceptance.md)
- [Implementation TODOs (reconstruction backlog)](implementation/todo/README.md)
- [Source tree reconstruction prompt template](tmp/src-recreate.md)
- [MMORPG Essentials (long-term targets)](design/mmorpg_elements.md)
- [Technical Architecture](technical/README.md)
- [Game Design](design/README.md)

## Reconstruction workflow (single prompt)

If you are reconstructing `src/` from scratch, treat this as the canonical flow:

1. Read `docs/policy/INSTRUCT.md` and obey all invariants.
2. Read `docs/decisions/0001-reconstruction-scope.md` to understand what “complete” means.
3. Use `docs/implementation/reconstruction_acceptance.md` as the checklist; do not claim “done” without checking every item.
4. Run the agent using `docs/tmp/src-recreate.md` as the reconstruction prompt.

If any leaf docs are not reachable via README TOCs, fix the TOCs first (docs-only change) before implementing.

## Sections

- [Design](design/README.md)
- [Technical Architecture](technical/README.md)
- [Implementation Notes](implementation/README.md)
- [Setup](setup/README.md)
- [Assets](assets/README.md)
- [Plans](plan/README.md)
- [Policy](policy/README.md)
- [Decisions](decisions/README.md)
- [Templates (Agent Prompts)](tmp/README.md)
