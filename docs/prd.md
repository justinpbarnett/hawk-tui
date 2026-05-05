# Hawk TUI PRD

## Problem Statement

AI coding agents frequently modify code across one or more local repositories. Reviewing those changes inside a normal terminal workflow is awkward: existing review tools are usually web-based, editor-specific, or optimized for pull requests rather than local AI-edited working trees. The user wants a fast, keyboard-driven, TUI-native review tool that opens beside any AI editor in tmux, shows local diffs clearly, lets the reviewer leave line-specific comments while moving hunk-by-hunk, and copies those comments as a precise prompt that can be pasted into pi, Claude Code, Codex CLI, Droid, or any other coding agent.

The tool must work both for a single Git repo and for workspace directories containing multiple nested repos, such as a parent project directory with separate client, server, and database repos underneath it.

## Solution

Build **Hawk TUI**: a standalone Rust TUI application packaged as `hawk-tui` with a `hawk` binary. Hawk opens in the current directory, auto-detects whether it should review one repo or aggregate nested repos, renders unified inline diffs with syntax highlighting and intraline change highlighting, supports Vim-style keyboard navigation, autosaves review comments, and exports selected comments as a markdown prompt.

Hawk is provider-agnostic. It does not integrate directly with any AI API. Instead, it produces a high-quality prompt containing file, line, side, hunk, and compact context references. The reviewer can copy that prompt at any point with a hotkey and paste it into any adjacent AI editor or terminal pane.

Hawk should be usable directly from the CLI and from tmux bindings, including split-pane and popup recipes.

## User Stories

1. As a developer reviewing AI-edited code, I want to launch `hawk` from the current directory, so that I can immediately review local changes.
2. As a tmux user, I want to bind Hawk to a leader hotkey, so that I can open review beside my AI editor without leaving my session.
3. As a tmux user, I want both split-pane and popup launch recipes, so that I can choose the layout that fits my workflow.
4. As a developer using multiple AI editors, I want Hawk to be standalone and provider-agnostic, so that it works with pi, Claude Code, Codex CLI, Droid, or no AI editor at all.
5. As a developer, I want `hawk` to review all working tree changes against `HEAD` by default, so that I see everything the AI modified.
6. As a developer, I want untracked non-ignored text/code files included by default, so that newly-created AI files are reviewed.
7. As a developer, I want to diff against another branch with `--base`, so that I can perform PR-style reviews locally.
8. As an advanced user, I want `--staged`, `--ref`, `--repo`, and `--workspace` options, so that I can control the review target when needed.
9. As a developer in a multi-repo workspace, I want Hawk to detect changed nested repos under the launch directory, so that I can review client, server, and database changes together.
10. As a developer in a parent repo with nested repos, I want nested repos treated as separate review units, so that files are not double-counted as parent untracked files.
11. As a reviewer, I want diffs grouped by repo and file, so that paths are unambiguous in workspace mode.
12. As a reviewer, I want unified diffs, so that changed lines appear directly above or below related lines.
13. As a reviewer, I want added lines highlighted green and removed lines highlighted red, so that I can scan changes quickly.
14. As a reviewer, I want intraline changed text highlighted more strongly, so that I can see exactly what changed at a glance.
15. As a reviewer, I want syntax highlighting based on detected language, so that code remains readable inside diffs.
16. As a reviewer, I want unknown languages to fall back to plain diff coloring, so that review never blocks on highlighting failures.
17. As a reviewer, I want common languages bundled, including Rust, TypeScript, TSX, JavaScript, JSX, Python, Go, PHP, SQL, JSON, YAML, TOML, Markdown, Shell, HTML, and CSS, so that my usual projects are highlighted.
18. As a reviewer, I want non-text files skipped with visible reasons, so that binary or media changes do not break the review flow.
19. As a reviewer, I want huge text/code diffs collapsed initially, so that large generated or lockfile changes do not freeze or dominate the UI.
20. As a reviewer, I want collapsed huge diffs to load only when I explicitly press Enter, so that navigation remains predictable.
21. As a reviewer, I want lockfiles included if they are text, so that dependency changes are still reviewable.
22. As a keyboard-first user, I want `j` and `k` to move between lines, so that navigation feels Vim-native.
23. As a keyboard-first user, I want `J`, `K`, and `Tab` to move between hunks with wrapping, so that I can review hunk-by-hunk quickly.
24. As a keyboard-first user, I want `e` to toggle a file sidebar, so that I can inspect changed files without leaving the diff.
25. As a reviewer, I want the sidebar to show repo/file path, added lines, removed lines, and comment count, so that I can see review scope and feedback density.
26. As a reviewer, I want `o` to open an inline comment editor below the current diff line, so that I can write feedback in place.
27. As a reviewer, I want `Esc` or `Ctrl-C` to leave comment editing mode and autosave, so that I can return to navigation quickly.
28. As a reviewer, I want multiline comments, so that I can describe larger hunk, method, or file issues from a line anchor.
29. As a reviewer, I want one comment per diff line anchor, so that `o` edits the existing comment instead of creating duplicate threads.
30. As a reviewer, I want comments only on actual diff content lines, so that references stay precise and simple.
31. As a reviewer, I want to comment on added, removed, and context lines, so that I can address new code, deleted behavior, or nearby unchanged code.
32. As a reviewer, I want removed-line comments to reference old paths/line numbers, so that deletion feedback is actionable.
33. As a reviewer, I want added-line comments to reference new paths/line numbers, so that implementation feedback is actionable.
34. As a reviewer, I want comments to autosave continuously, so that tmux kills or accidental exits do not lose work.
35. As a reviewer, I want sessions stored outside the repo under global state, so that Hawk does not create project files.
36. As a reviewer, I want sessions keyed by workspace root, so that multi-repo review state resumes correctly.
37. As a reviewer, I want cursor position and visited hunks autosaved, so that I can resume where I left off.
38. As a reviewer, I want lightweight review progress, so that I know how much of the diff I have visited.
39. As a reviewer, I want `y` to copy only uncopied comments, so that I can send new feedback without repeating old instructions.
40. As a reviewer, I want `Y` to copy all visible comments, so that I can resend the full review if needed.
41. As a Vim user, I want `:w` to copy uncopied comments and `:w!` to copy all comments, so that prompt export matches familiar save semantics.
42. As a reviewer, I want copied comments marked as `copied`, so that I know what has already been exported.
43. As a reviewer, I want prompts grouped by repo and file, so that the AI can locate each comment unambiguously.
44. As a reviewer, I want each prompt item to include side-aware file/line references and compact context, so that the AI can address comments accurately without excessive prompt bloat.
45. As a reviewer, I want compact context to include the hunk header and a few surrounding lines, so that references remain robust if line numbers drift.
46. As a reviewer, I want clipboard export to try system clipboard, then OSC52, then a temp file, so that prompt export works across terminals, tmux, and SSH-like environments.
47. As a reviewer, I want comments marked copied only after export succeeds, so that status reflects reality.
48. As a reviewer, I want `r` to reload the diff manually, so that I control when the UI reconciles after AI edits.
49. As a reviewer, I want Hawk to detect live file changes and show a dirty indicator, so that I know when the displayed diff is stale.
50. As a reviewer, I do not want automatic reloads while editing comments, so that the UI does not jump unexpectedly.
51. As a reviewer, I want copied comments to auto-resolve and hide when their target hunk or line changes after reload, so that addressed feedback disappears with minimal manual work.
52. As a reviewer, I want auto-resolved comments kept in recoverable session history, so that mistaken resolution is not permanent.
53. As a reviewer, I want draft comments to remain visible across reloads, so that unsent feedback is not lost.
54. As a reviewer, I want stale comments to remain visible when Hawk cannot safely map them, so that uncertain state is explicit.
55. As a reviewer, I want `m` to manually toggle resolved state, so that I can clean up comments myself when needed.
56. As a reviewer, I want `s` to show or hide resolved comments, so that I can inspect history without cluttering the normal view.
57. As a reviewer, I want `n` to jump to the next visible unresolved comment, so that I can quickly review feedback.
58. As a reviewer, I want `p` and `N` to jump to the previous visible unresolved comment, so that comment navigation is symmetric.
59. As a reviewer, I want `c` to open a focused comment list, so that I can see all feedback at once.
60. As a reviewer, I want `j` and `k` inside the comment list to move selection, so that the list remains keyboard-native.
61. As a reviewer, I want Enter in the comment list to jump to the selected comment, so that I can inspect it in context.
62. As a reviewer, I want `x` to delete the current comment, so that I can remove feedback that no longer applies.
63. As a reviewer, I want `X` to delete all visible comments with confirmation, so that I can clear the current visible review state safely.
64. As a reviewer, I want `:reset` to clear the current workspace session with confirmation, so that I can recover from stale or unwanted state.
65. As a reviewer, I want `:clear-resolved` eventually, so that I can clean session history without deleting active comments.
66. As a reviewer, I want `q` and `Ctrl-C` in navigation mode to quit, so that exiting is easy.
67. As a reviewer, I want quit confirmation only when uncopied draft comments exist, so that Hawk prevents forgotten feedback without unnecessary friction.
68. As a reviewer, I want `?` help, so that I can discover the fixed keybindings.
69. As a reviewer, I want command mode with a small Vim-like command set, so that common actions are familiar without a large command language.
70. As a reviewer, I want repo and file errors shown as visible placeholders, so that one broken repo does not hide incomplete coverage.
71. As a reviewer, I want no-change repos hidden by default but represented in workspace summaries when useful, so that noise stays low.
72. As a reviewer, I want rename detection, so that moved files are displayed as renames rather than unrelated deletes/adds when possible.
73. As a reviewer, I want comments on deleted files to be valid, so that I can question removed behavior even when the file no longer exists.
74. As a developer, I want Hawk to remain local and not publish GitHub/GitLab reviews in MVP, so that setup stays simple and private.
75. As a developer, I want no mouse requirement in MVP, so that the tool remains reliable in terminal and tmux workflows.
76. As a developer, I want Hawk not to edit source files or launch `$EDITOR` in MVP, so that it stays focused on review and prompt export.
77. As a developer, I want optional config for thresholds and excludes, so that I can tune performance without configuring every key.
78. As a developer, I want fixed keybindings in MVP, so that implementation focuses on core workflow rather than keymap customization.
79. As a developer, I want `hawk` installable with `cargo install --path .` during development, so that local iteration is simple.
80. As a future public user, I want side-by-side diff and packaging options later, so that Hawk can grow beyond the initial local workflow.

## Implementation Decisions

- The project will be a new Rust codebase named `hawk-tui` with a `hawk` binary.
- Hawk will be a standalone TUI app built with Ratatui and crossterm.
- Hawk will use a deep-module architecture with small public surfaces. Complex behavior should be encapsulated behind stable, testable interfaces.
- The TUI should primarily interact with a single backend facade, tentatively called the review engine.
- The review engine owns workspace discovery, diff loading, session state, comment persistence, reload reconciliation, and prompt export.
- The TUI owns transient interaction state such as cursor position, scroll offset, active panel, comment editing mode, help overlay, and command mode.
- Git diff generation will shell out to the Git CLI for MVP rather than use libgit2.
- Default diff mode is all tracked changes against `HEAD` plus untracked non-ignored text/code files.
- PR-style review is supported through a base branch option.
- Advanced explicit diff refs are supported through a raw ref/range option.
- Workspace mode is auto-detected by default and can be forced or disabled with flags.
- Workspace discovery scans downward from the launch directory for nested Git repos, skips common dependency/cache directories, deduplicates roots, and treats nested repos as separate review units.
- Parent repo untracked scanning must exclude nested repo directories to avoid double-counting.
- Repos with errors are represented as visible skipped/error placeholders instead of failing the whole workspace.
- Unified diff is the only MVP diff layout.
- Rename detection should be enabled and represented explicitly where practical.
- Tree-sitter or a Rust-compatible tree-sitter highlighting stack will be used for syntax highlighting.
- Bundled MVP grammars include Rust, TypeScript, TSX, JavaScript, JSX, Python, Go, PHP, SQL, JSON, YAML, TOML, Markdown, Shell/Bash, HTML, and CSS.
- Syntax highlighting is best-effort; highlighting failures fall back to plain text plus diff coloring.
- Intraline changed-text highlighting is required for MVP.
- Intraline diffing should pair related removed/added lines where possible and fall back to full-line highlighting when ambiguous.
- Non-text files are skipped conservatively using Git binary detection, NUL-byte checks, extension/MIME hints, and UTF-8 or mostly-valid text sampling.
- Huge text/code diffs are collapsed and lazy-loaded only after explicit user action.
- Default thresholds are configurable for eager file size, eager diff lines, and absolute file size.
- Comments attach to a rich line anchor containing repo path, file path, side, old/new line numbers, hunk header, original line text, and nearby context hash.
- Comments may attach to added, removed, or context lines.
- Comments cannot attach to headers, placeholders, skipped files, or other non-diff rows in MVP.
- MVP allows one multiline comment per line anchor.
- Comment statuses include at least draft, copied, resolved, and stale.
- Prompt copy marks included draft comments as copied only after successful clipboard, OSC52, or temp-file export.
- Copy metadata should include batch id, timestamp, scope, comment ids, and prompt hash, but should not store full prompt text by default.
- Sessions are stored globally under the user state directory for Hawk, keyed by canonical workspace root.
- Autosave occurs after every meaningful review-state mutation, including comment edits, deletion, status changes, cursor/progress updates, and copy metadata changes.
- Live file watching sets a dirty indicator but does not automatically reload or reconcile the diff.
- Manual reload recomputes diffs and reconciles comments.
- Copied comments whose target hunk/line changed or disappeared after reload are automatically marked resolved and hidden by default.
- Draft comments are not auto-resolved merely because a diff changed.
- Resolved comments remain recoverable in session history.
- Prompt export includes only comments, not review progress summaries.
- Prompt export groups comments by repo and file, includes side-aware file/line references, and includes compact context around the referenced line.
- Keybindings are fixed in MVP and may become configurable later.
- The file sidebar is toggled with `e` and shows repo/file paths, added lines, removed lines, and comment count.
- The comment list is toggled with `c`, receives focus, uses `j/k` navigation, and jumps to a selected comment with Enter.
- The MVP command mode is intentionally small: copy uncopied, copy all, quit, forced quit, reload, reset, and potentially clear resolved.
- Clipboard export order is system clipboard, OSC52, then temp-file fallback.
- The MVP does not auto-paste into tmux panes, but reserves future support through `P` or command-mode paste commands.
- Hawk remains provider-agnostic and does not call AI APIs.
- Early install path is `cargo install --path .`; public Cargo/Homebrew packaging is out of scope for initial implementation.

## Testing Decisions

- Core logic should be well-tested from the start; TUI rendering tests should stay light until behavior stabilizes.
- Tests should assert external behavior and contracts rather than private implementation details.
- Diff parser tests should cover added, removed, and context lines; multiple files; multiple hunks; new files; deleted files; no-newline markers; and rename metadata where supported.
- Untracked file tests should verify non-ignored text files are included and ignored files are excluded.
- Workspace discovery tests should cover single repo mode, nested repo mode, parent plus child repos, deduplication, skipped directories, and parent untracked exclusion for nested repos.
- Text detection tests should cover binary files, NUL-byte files, UTF-8 text files, mostly-valid text files, known non-text extensions, lockfiles, and oversized files.
- Lazy-loading tests should verify huge diffs are represented as collapsed placeholders and only load on explicit action.
- Intraline diff tests should verify changed spans for paired removed/added lines and fallback behavior for ambiguous pairs.
- Session tests should cover autosave/load, workspace keying, line anchor serialization, comment status transitions, copy batch metadata, visited hunk persistence, and reset behavior.
- Reconciliation tests should verify copied comments auto-resolve when their target changes or disappears, draft comments remain visible, and uncertain mappings become stale.
- Prompt tests should verify uncopied-only export, all-visible export, grouping by repo/file, side-aware references, compact context, deleted-file references, removed-line references, and zero-comment no-op behavior.
- Clipboard tests should use abstraction/fakes to verify fallback ordering and status mutation only on successful export.
- TUI state tests should cover key-driven transitions such as navigation mode to comment editor, saving on Escape, sidebar toggle, comment list focus, command mode commands, quit confirmation, and delete confirmation.
- End-to-end tests should create temporary Git repos and nested workspace repos to validate realistic default, staged, base, ref, and workspace review flows without relying on a user machine's real repos.

## Out of Scope

- GitHub, GitLab, or hosted review publishing.
- Built-in AI provider integrations.
- Automatic paste into adjacent tmux panes for MVP.
- Side-by-side diff view.
- Mouse support.
- Source file editing or launching `$EDITOR`.
- Fully configurable keybindings.
- Framework-aware sidebar grouping.
- Storing full copied prompt history by default.
- Public package distribution through Homebrew or crates.io as part of the first milestone.
- Comments on whole files or whole hunks as first-class anchor types.
- Multiple comments/threads on the same line.
- Automatic reload/reflow while the user is editing.

## Further Notes

- The first implementation milestone should be a vertical tracer bullet that can open a real workspace, discover changed repos, parse tracked and untracked diffs, render unified hunks, navigate with the core keys, add/edit/delete inline comments, autosave sessions, copy a prompt, and resume after restart.
- MVP hardening should then add tree-sitter highlighting, intraline highlighting, sidebar metrics, comment list, live dirty detection, reload/reconciliation, huge diff lazy loading, clipboard fallbacks, config loading, and tmux documentation.
- Hawk's public positioning should be a general local code review TUI, with AI-oriented prompt export as the killer feature.
- The name `hawk-tui` is intentionally used for the project/package, with `hawk` as the binary name.
