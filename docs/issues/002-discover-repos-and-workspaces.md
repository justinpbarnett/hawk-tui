# Discover single repos and nested workspaces

**Type:** AFK  
**User stories covered:** 1, 5, 9, 10, 11, 35, 36, 70, 71

## What to build

Teach Hawk to open the current directory as either a single repo review or an auto-detected workspace review. A workspace may contain multiple nested Git repos; Hawk should discover changed repos, dedupe roots, avoid double-counting nested repos as parent untracked files, and represent repo-level errors without failing the whole review.

This slice should expose workspace discovery through the review engine and show discovered repos in the TUI, even if diff rendering is still minimal.

## Acceptance criteria

- [ ] Launching inside a single Git repo detects that repo as the review target.
- [ ] Launching from a directory containing nested Git repos detects changed child repos.
- [ ] Launching from a parent repo containing nested repos treats nested repos as separate review units.
- [ ] Nested repo paths are excluded from parent repo untracked scanning.
- [ ] Common dependency/cache directories are skipped during discovery.
- [ ] Duplicate repo roots are deduped.
- [ ] Repo errors are represented as visible review items/placeholders.
- [ ] No-change repos do not clutter the default review stream.
- [ ] Workspace discovery has tests covering single repo, nested repo, parent plus child repo, dedupe, and skip rules.

## Blocked by

- [001 - Scaffold `hawk` tracer bullet](001-scaffold-hawk-tracer-bullet.md)
