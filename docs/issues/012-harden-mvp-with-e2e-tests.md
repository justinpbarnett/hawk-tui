# Harden MVP with end-to-end tests

**Type:** AFK  
**User stories covered:** 1-79

## What to build

Stabilize Hawk as an MVP by adding realistic end-to-end tests, filling behavioral gaps, fixing UX rough edges, and ensuring the complete local review loop works in both single-repo and nested-workspace scenarios.

The target demo is: launch Hawk from a workspace, discover changed repos, show tracked and untracked diffs, navigate hunks, add comments, autosave, copy a prompt, reload after code changes, auto-resolve copied comments, and resume cleanly after restart.

## Acceptance criteria

- [ ] End-to-end tests create temporary single-repo and nested-workspace fixtures.
- [ ] E2E tests cover default working tree review with tracked and untracked files.
- [ ] E2E tests cover workspace aggregation across multiple nested repos.
- [ ] E2E tests cover `--staged`, `--base`, and `--ref` flows.
- [ ] E2E tests cover adding comments, autosave, restart/resume, prompt export, and copied status.
- [ ] E2E tests cover reload reconciliation and copied-comment auto-resolution.
- [ ] E2E tests cover skipped repo/file placeholders and huge diff lazy loading.
- [ ] Manual smoke test docs describe the expected tmux split and popup workflow.
- [ ] All public-facing docs use the `hawk-tui` project name and `hawk` binary name consistently.
- [ ] MVP scope remains provider-agnostic, local-only, keyboard-only, unified-diff-only, and non-editing.
- [ ] Formatting, linting, and tests pass consistently.

## Blocked by

- [006 - Export comments as AI prompt](006-export-comments-as-ai-prompt.md)
- [008 - Reload changed diffs and reconcile comments](008-reload-and-reconcile-comments.md)
- [009 - Add syntax and intraline highlighting](009-add-syntax-and-intraline-highlighting.md)
- [011 - Add config, CLI modes, and tmux docs](011-config-cli-modes-and-tmux-docs.md)
