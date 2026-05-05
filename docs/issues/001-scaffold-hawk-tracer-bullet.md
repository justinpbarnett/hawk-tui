# Scaffold `hawk` tracer bullet

**Type:** AFK  
**User stories covered:** 1, 4, 12, 22, 23, 26, 27, 66, 68, 74, 75, 76, 79

## What to build

Create the initial Rust `hawk-tui` project with a `hawk` binary and a minimal end-to-end TUI flow. The first demo should launch from a terminal, enter raw terminal mode safely, show a placeholder review document, handle core navigation keys, show help, and exit cleanly.

The implementation should establish the deep-module architecture: a small review engine facade for core behavior and a TUI layer that owns transient interaction state.

## Acceptance criteria

- [ ] `cargo run` starts a `hawk` TUI binary.
- [ ] Terminal setup/teardown is safe, including panic/error cleanup where practical.
- [ ] The app shows a minimal review screen with status bar and help overlay.
- [ ] `j/k`, `J/K`, `Tab`, `?`, `q`, `Ctrl-C`, and `Esc` have initial state-machine behavior.
- [ ] The TUI talks to a small backend facade rather than directly reaching into low-level modules.
- [ ] The codebase has an initial module layout that keeps core review logic separate from Ratatui rendering.
- [ ] `cargo test` and formatting/lint basics pass.

## Blocked by

None - can start immediately.
