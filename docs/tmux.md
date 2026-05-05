# Hawk TUI tmux usage

Install during development:

```sh
cargo install --path .
```

Run directly:

```sh
hawk
hawk --staged
hawk --base main
hawk --ref 'main..HEAD'
hawk --repo
hawk --workspace
```

Split-pane binding for an installed `hawk`:

```tmux
bind-key H split-window -h -c '#{pane_current_path}' 'hawk'
```

Split-pane binding for this local checkout:

```tmux
bind-key g split-window -h -p 70 -c '#{pane_current_path}' 'cargo run --manifest-path /Users/justin.barnett/dev/hawk-tui/Cargo.toml --'
```

Popup binding:

```tmux
bind-key h display-popup -E -w 90% -h 90% -d '#{pane_current_path}' 'hawk'
```

Hawk stays local and provider-agnostic. Copy comments with `y`/`Y` or `:w`/`:w!`, then paste the generated prompt into pi, Claude Code, Codex CLI, Droid, or any other agent.
