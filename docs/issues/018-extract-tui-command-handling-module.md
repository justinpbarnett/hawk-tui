# Extract TUI command handling Module

**Type:** AFK  
**User stories covered:** 22-29, 39-41, 55-69

## What to build

Extract TUI interaction semantics into a command handling Module. Key events and command-mode input should produce review actions or state transitions, while rendering stays focused on drawing the current state.

The Module should preserve fixed MVP keybindings and command behavior while improving locality for navigation mode, editing mode, command mode, comment list mode, confirmation flows, quit confirmation, and status messages.

## Acceptance criteria

- [ ] Key handling and Vim-like command handling move out of the rendering module into a command handling Module.
- [ ] The command handling interface accepts input plus current app/review state and returns explicit actions or state updates.
- [ ] Rendering code no longer owns review mutation decisions.
- [ ] Existing keybindings remain unchanged: navigation, hunk jumps, help, comments, copy, reload, sidebar, comment list, delete, reset, quit, and command mode.
- [ ] Quit confirmation still appears only when uncopied draft comments exist.
- [ ] Tests cover navigation mode, comment editing autosave, command mode commands, unknown command feedback, comment list focus/jump, delete confirmation, reset, reload, and quit confirmation through the command handling interface.

## Blocked by

- [013 - Introduce review document Module for row navigation and anchors](013-introduce-review-document-module.md)
- [016 - Introduce session store Module for autosave and status transitions](016-introduce-session-store-module.md)
- [017 - Extract prompt export pipeline Module](017-extract-prompt-export-pipeline-module.md)
