# Render navigable unified diff TUI

**Type:** AFK  
**User stories covered:** 11, 12, 13, 22, 23, 24, 30, 66, 67, 68, 69, 75, 76

## What to build

Render parsed diffs as a keyboard-navigable unified diff review stream. The reviewer should move line-by-line and hunk-by-hunk, with hunk navigation wrapping at boundaries. The UI should make the current repo, file, hunk, dirty/review status, and active mode visible.

This slice should provide line-level red/green diff coloring, but full syntax and intraline highlighting can be added in a later slice.

## Acceptance criteria

- [ ] Parsed repo/file/hunk/diff-line rows render in a unified diff stream.
- [ ] Added lines have green diff styling and removed lines have red diff styling.
- [ ] Context lines render normally.
- [ ] Current line is visibly selected.
- [ ] `j/k` move between rendered lines.
- [ ] `J/K` move between hunks and wrap at the end/start.
- [ ] `Tab` jumps to the next hunk and wraps.
- [ ] `q` and `Ctrl-C` quit in navigation mode.
- [ ] `?` displays the current fixed keybindings.
- [ ] `:` command mode supports at least `:q`, `:q!`, and unknown-command feedback.
- [ ] TUI state transition tests cover navigation, hunk wrapping, help, command mode, and quit confirmation behavior.

## Blocked by

- [003 - Load tracked and untracked diffs](003-load-tracked-and-untracked-diffs.md)
