# Skip non-text files and lazy-load huge diffs

**Type:** AFK  
**User stories covered:** 18, 19, 20, 21, 30, 70

## What to build

Add conservative text detection, visible skipped-file placeholders, and lazy loading for huge text/code diffs. Non-text files should be skipped with explicit reasons. Huge text/code diffs should appear collapsed by default and load only when the user explicitly presses Enter on the placeholder.

This slice protects review performance and keeps noisy files from overwhelming the main diff stream.

## Acceptance criteria

- [ ] Git binary diffs are skipped with visible reasons.
- [ ] Files with NUL bytes or known non-text media/archive/font/database extensions are skipped.
- [ ] UTF-8 and mostly-valid text files are included.
- [ ] Text lockfiles are included unless they exceed configured eager thresholds.
- [ ] Huge text/code diffs over eager thresholds render as collapsed placeholders.
- [ ] Collapsed placeholders show repo/file path, added count, removed count, and reason.
- [ ] Pressing Enter on a collapsed huge diff explicitly loads and renders that file's diff.
- [ ] Hunk navigation does not auto-load huge diffs merely by passing over them.
- [ ] Files over absolute size limit remain skipped unless config/flags raise the limit.
- [ ] Tests cover binary detection, text detection, known non-text extensions, lockfiles, collapsed placeholders, explicit load, and absolute limit behavior.

## Blocked by

- [003 - Load tracked and untracked diffs](003-load-tracked-and-untracked-diffs.md)
- [004 - Render navigable unified diff TUI](004-render-navigable-unified-diff-tui.md)
