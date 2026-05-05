# Hawk TUI

Hawk TUI is a local, keyboard-first code review TUI for inspecting Git working-tree changes and exporting line-specific review comments as an AI-ready prompt.

## Development

```sh
cargo test
cargo run -- --no-tui
cargo run
```

Install locally:

```sh
cargo install --path .
```

Run from any Git repo or workspace:

```sh
hawk
hawk --staged
hawk --base main
hawk --ref 'main..HEAD'
hawk --repo
hawk --workspace
```

See `docs/prd.md` for product scope and `docs/tmux.md` for tmux bindings.
