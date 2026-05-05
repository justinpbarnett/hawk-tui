# Load tracked and untracked diffs

**Type:** AFK  
**User stories covered:** 5, 6, 7, 8, 11, 12, 31, 32, 33, 70, 72, 73

## What to build

Implement Git CLI-backed diff loading and unified diff parsing for each discovered repo. The default review target is all tracked changes against `HEAD` plus untracked non-ignored text files. The diff model should preserve repo path, file path, old/new paths for renames, hunks, old/new line numbers, side, hunk headers, and raw line text.

The TUI should render real parsed diff rows in a simple plain style. Highlighting polish can come later.

## Acceptance criteria

- [ ] Default mode loads tracked working tree changes against `HEAD`.
- [ ] Default mode includes untracked non-ignored files as new-file diffs.
- [ ] Ignored files are excluded.
- [ ] Unified diffs parse into repos, files, hunks, and side-aware diff lines.
- [ ] Added, removed, and context lines preserve correct old/new line numbers.
- [ ] Deleted-file and removed-line references are representable.
- [ ] Rename detection is enabled and represented where practical.
- [ ] Repo-level Git errors appear as review placeholders rather than panicking.
- [ ] Parser tests cover added/removed/context lines, multiple files, multiple hunks, new files, deleted files, no-newline markers, and rename metadata where supported.

## Blocked by

- [002 - Discover single repos and nested workspaces](002-discover-repos-and-workspaces.md)
