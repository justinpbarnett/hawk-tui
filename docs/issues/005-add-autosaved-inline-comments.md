# Add autosaved inline comments

**Type:** AFK  
**User stories covered:** 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 62, 63, 64, 67

## What to build

Add line-anchored review comments with inline editing and autosaved session persistence. Comments attach to added, removed, or context diff lines using a rich anchor with repo path, file path, side, old/new line numbers, hunk header, original line text, and nearby context hash.

Sessions should be stored globally under Hawk's user state directory, keyed by canonical workspace root. Hawk should resume comments, cursor position, and visited hunk progress after restart.

## Acceptance criteria

- [ ] Pressing `o` on a diff content line opens an inline multiline comment editor below that line.
- [ ] `Esc` or `Ctrl-C` exits comment editing mode and autosaves the comment.
- [ ] Pressing `o` again on the same anchored line edits the existing comment.
- [ ] Emptying a comment deletes it.
- [ ] Pressing `x` deletes the current line's comment.
- [ ] Pressing `X` asks for confirmation and deletes all visible comments.
- [ ] Comments cannot be added to headers, placeholders, or skipped file rows; Hawk shows a status hint instead.
- [ ] Comments support added, removed, and context line anchors with side-aware line references.
- [ ] Sessions autosave comments, cursor position, visited hunks, and workspace metadata.
- [ ] Restarting Hawk in the same workspace resumes the saved review session.
- [ ] `:reset` clears the current workspace session after confirmation.
- [ ] Session tests cover save/load, anchor serialization, one-comment-per-anchor behavior, delete behavior, visited hunk persistence, and reset behavior.

## Blocked by

- [004 - Render navigable unified diff TUI](004-render-navigable-unified-diff-tui.md)
