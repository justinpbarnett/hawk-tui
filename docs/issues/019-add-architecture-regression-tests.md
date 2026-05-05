# Add architecture regression tests for deepened Modules

**Type:** AFK  
**User stories covered:** Testing decisions, 1-79 indirectly

## What to build

Add regression tests that lock in the deeper Module interfaces introduced by the architecture refactors. These tests should verify behavior through public interfaces rather than implementation details, so future refactors preserve locality and leverage.

The tests should complement existing end-to-end temp-repo tests and make architectural regressions visible when behavior leaks back into callers.

## Acceptance criteria

- [ ] Review document tests cover row construction, anchor lookup, hunk navigation, visible-comment filtering, and comment jumps.
- [ ] Git adapter tests cover command success, command failure, repo root discovery, changed-state detection, diff generation, untracked listing, and dirty fingerprint inputs.
- [ ] Diff loading tests cover review-target semantics separately from parser tests.
- [ ] Session store tests cover autosave-on-mutation, reset, status transitions, copied metadata, and resolved/stale behavior.
- [ ] Prompt export pipeline tests cover destination fallback and success/failure mutation invariants.
- [ ] TUI command handling tests cover key and command semantics without requiring terminal rendering.
- [ ] End-to-end tests still cover realistic default, staged, base/ref, workspace, comments, export, reload, skipped file, and huge diff flows.
- [ ] `cargo test`, `cargo fmt`, and `cargo clippy -- -D warnings` pass.

## Blocked by

- [013 - Introduce review document Module for row navigation and anchors](013-introduce-review-document-module.md)
- [014 - Add Git adapter Module for repo discovery, diff loading, and dirty checks](014-add-git-adapter-module.md)
- [015 - Deepen diff loading Module around review-target semantics](015-deepen-diff-loading-module.md)
- [016 - Introduce session store Module for autosave and status transitions](016-introduce-session-store-module.md)
- [017 - Extract prompt export pipeline Module](017-extract-prompt-export-pipeline-module.md)
- [018 - Extract TUI command handling Module](018-extract-tui-command-handling-module.md)
