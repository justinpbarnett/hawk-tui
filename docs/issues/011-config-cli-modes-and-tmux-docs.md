# Add config, CLI modes, and tmux docs

**Type:** AFK  
**User stories covered:** 2, 3, 7, 8, 46, 77, 78, 79

## What to build

Round out Hawk's external interface with optional config, documented CLI modes, and tmux launch recipes. The app should work without a config file, but users should be able to tune diff thresholds, untracked-file behavior, workspace excludes, and clipboard preferences.

The docs should show direct CLI usage plus tmux split-pane and popup bindings.

## Acceptance criteria

- [ ] `hawk` default mode reviews all changes against `HEAD`, including untracked non-ignored text/code files.
- [ ] `--staged` reviews staged changes only.
- [ ] `--base <branch>` reviews PR-style diffs with `branch...HEAD` per repo.
- [ ] `--ref <range>` passes an explicit advanced Git diff ref/range.
- [ ] `--repo` forces only the containing/current repo.
- [ ] `--workspace` forces nested workspace scanning from cwd.
- [ ] Missing base branches or bad refs become visible repo-level errors in workspace mode.
- [ ] Optional config loads from the Hawk config directory if present.
- [ ] Config supports diff context lines, eager/absolute size thresholds, include-untracked behavior, workspace exclude dirs, and clipboard preference.
- [ ] CLI flags override config where applicable.
- [ ] Docs include `cargo install --path .` development install instructions.
- [ ] Docs include tmux split-pane and popup binding examples.
- [ ] CLI/config tests cover default, staged, base, ref, repo/workspace override, config loading, and flag precedence.

## Blocked by

- [002 - Discover single repos and nested workspaces](002-discover-repos-and-workspaces.md)
- [003 - Load tracked and untracked diffs](003-load-tracked-and-untracked-diffs.md)
- [010 - Skip non-text files and lazy-load huge diffs](010-skip-non-text-and-lazy-load-huge-diffs.md)
