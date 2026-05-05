# Add syntax and intraline highlighting

**Type:** AFK  
**User stories covered:** 13, 14, 15, 16, 17

## What to build

Upgrade diff rendering with best-effort tree-sitter syntax highlighting and required intraline changed-text highlighting. Added and removed lines should retain full-line green/red diff styling, while changed substrings inside paired lines receive stronger/darker emphasis.

The renderer must remain robust: unknown languages, parser failures, and ambiguous intraline pairing should fall back cleanly to readable diff coloring.

## Acceptance criteria

- [ ] Tree-sitter-based syntax highlighting is integrated behind a small renderer/highlighter interface.
- [ ] MVP bundled grammars include Rust, TypeScript, TSX, JavaScript, JSX, Python, Go, PHP, SQL, JSON, YAML, TOML, Markdown, Shell/Bash, HTML, and CSS.
- [ ] Language detection uses file path/extension and falls back gracefully.
- [ ] Added and removed lines keep full-line diff styling.
- [ ] Related removed/added lines receive intraline changed-span highlighting.
- [ ] Ambiguous pairings fall back to full-line add/remove highlighting.
- [ ] Syntax and diff styling compose without making text unreadable.
- [ ] Highlighting failures do not crash or block review.
- [ ] Tests cover language fallback, intraline spans, ambiguous fallback, and rendering span composition at the model/style level.

## Blocked by

- [004 - Render navigable unified diff TUI](004-render-navigable-unified-diff-tui.md)
