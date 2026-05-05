# Extract prompt export pipeline Module

**Type:** AFK  
**User stories covered:** 39-47

## What to build

Create a prompt export pipeline Module that owns the full export path: selecting eligible comments, building the markdown prompt, attempting clipboard/OSC52/temp-file destinations in order, recording copy batch metadata, and marking comments copied only after success.

The review engine should call one export interface and receive a clear result for the TUI status area. Prompt formatting and destination fallback should remain independently testable behind internal seams.

## Acceptance criteria

- [ ] One prompt export interface handles uncopied-only and all-visible export scopes.
- [ ] Zero-comment export attempts do not write to any destination and do not mutate session state.
- [ ] Markdown prompt output remains grouped by repo and file with side-aware references and compact hunk context.
- [ ] Destination fallback order remains system clipboard, OSC52, then temp file.
- [ ] Comments are marked copied only after a destination succeeds.
- [ ] Copy batch metadata records timestamp, scope, comment ids, and prompt hash without storing full prompt text.
- [ ] Tests use fake destinations to cover selection scope, prompt grouping, side-aware references, fallback ordering, success mutation, failure non-mutation, and zero-comment no-op behavior.

## Blocked by

- [016 - Introduce session store Module for autosave and status transitions](016-introduce-session-store-module.md)
