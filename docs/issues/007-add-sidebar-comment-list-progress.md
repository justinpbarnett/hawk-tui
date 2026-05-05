# Add sidebar, comment list, and review progress

**Type:** AFK  
**User stories covered:** 24, 25, 37, 38, 55, 56, 57, 58, 59, 60, 61, 68

## What to build

Add review navigation surfaces beyond the main diff: a file sidebar toggled with `e`, a focused comment list toggled with `c`, comment jump keys, resolved-comment visibility, and lightweight visited-hunk progress.

The sidebar should group by repo and show file-local path, added lines, removed lines, and comment count. The comment list should show visible comments and allow keyboard selection and jump-to-comment.

## Acceptance criteria

- [ ] Pressing `e` toggles a file sidebar.
- [ ] The sidebar groups files by repo in workspace mode.
- [ ] Sidebar rows show file-local path, added count, removed count, and comment count.
- [ ] Pressing `c` toggles a focused comment list overlay or drawer.
- [ ] In the comment list, `j/k` move selection.
- [ ] In the comment list, Enter jumps to the selected comment and closes the list.
- [ ] In the comment list, Esc closes the list.
- [ ] In navigation mode, `n` jumps to the next visible unresolved comment and wraps.
- [ ] In navigation mode, `p` and `N` jump to the previous visible unresolved comment and wrap.
- [ ] `m` toggles the current comment's resolved state.
- [ ] `s` toggles showing/hiding resolved comments.
- [ ] Hunk visit progress is tracked and shown in the status area.
- [ ] Progress is autosaved and restored with the session.
- [ ] TUI state tests cover sidebar toggle, comment list focus, comment jumping, resolved visibility, and progress tracking.

## Blocked by

- [005 - Add autosaved inline comments](005-add-autosaved-inline-comments.md)
