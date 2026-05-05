# Add Git adapter Module for repo discovery, diff loading, and dirty checks

**Type:** AFK  
**User stories covered:** 5-10, 48-49, 70-72

## What to build

Introduce a Git adapter Module as the single seam for Git CLI behavior. Repo root discovery, changed-repo detection, status checks, diff generation, untracked-file listing, and dirty fingerprints should go through this Module instead of scattered `Command::new("git")` calls.

The production adapter should still shell out to Git for MVP. Tests may use a fake adapter where behavior matters more than Git itself, and temp repos where realistic Git behavior is required.

## Acceptance criteria

- [ ] All direct Git process calls move behind a Git adapter interface.
- [ ] Workspace discovery uses the Git adapter for repo root and changed-state checks.
- [ ] Diff loading uses the Git adapter for tracked diffs and untracked-file listing.
- [ ] Dirty detection uses the Git adapter instead of directly shelling out from the review engine.
- [ ] Git errors are represented as visible repo-level review placeholders where existing behavior requires it.
- [ ] Tests cover success and failure paths using a fake adapter or temp-repo adapter as appropriate.
- [ ] Existing end-to-end temp-repo tests continue to pass.

## Blocked by

None - can start immediately.
