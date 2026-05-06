# Hawk TUI tmux usage

Install dependencies during development:

```sh
npm install
```

Run directly:

```sh
./bin/hawk
./bin/hawk --staged
./bin/hawk --base main
./bin/hawk --ref 'main..HEAD'
./bin/hawk --repo
./bin/hawk --workspace
```

Split-pane binding for an installed `hawk`:

```tmux
bind-key H split-window -h -c '#{pane_current_path}' 'hawk'
```

Split-pane binding for this local checkout, with Hawk taking 60% of the screen:

```tmux
bind-key g split-window -h -p 60 -c '#{pane_current_path}' '/Users/justin.barnett/dev/hawk-tui/bin/hawk'
```

Popup binding:

```tmux
bind-key h display-popup -E -w 90% -h 90% -d '#{pane_current_path}' 'hawk'
```

Hawk stays local and provider-agnostic. Copy comments with `y`/`Y`, then paste the generated prompt into pi, Claude Code, Codex CLI, Droid, or any other agent.
