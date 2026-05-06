# Hawk TUI

Hawk TUI is a local, keyboard-first code review TUI for inspecting Git working-tree changes and exporting line-specific review comments as an AI-ready prompt.

Hawk is now implemented in TypeScript with an OpenTUI shell.

## Requirements

- Bun
- Git

If Bun is installed at `~/.bun/bin/bun`, either add that directory to `PATH` or use the local launcher in `bin/hawk`.

## Development

```sh
npm install
npm test
npm run typecheck
```

Run the OpenTUI app:

```sh
./bin/hawk
# or
~/.bun/bin/bun src-ts/opentui.ts
```

Inspect the review document without launching the TUI:

```sh
./bin/hawk --no-tui
npm run hawk:no-tui
```

## Usage

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
