# Introduce session store Module for autosave and status transitions

**Type:** AFK  
**User stories covered:** 34-38, 42, 47, 51-56, 62-64

## What to build

Create a session store Module that owns session persistence, workspace keying, autosave ordering, reset behavior, comment mutation, and comment status transitions. The review engine and TUI should stop directly mutating session internals for operations like upsert, delete, copied, resolved, stale, reset, and clear-resolved.

The Module should make the autosave invariant explicit: meaningful review-state mutations are persisted, and copied status changes happen only after successful export.

## Acceptance criteria

- [ ] Session path/keying, load, save, and reset are exposed through a session store interface.
- [ ] Comment upsert, delete-current, delete-visible/all, toggle-resolved, clear-resolved, and show-resolved changes go through the session store Module.
- [ ] Copy batch metadata and copied-status transition are owned by the session store Module or a clear collaborator behind it.
- [ ] Callers no longer directly mutate `session.comments` for normal review operations.
- [ ] Autosave occurs after comment edits, deletes, status changes, reset, progress updates, and copy metadata changes.
- [ ] Tests cover workspace keying, save/load, reset, one-comment-per-anchor behavior, delete behavior, status transitions, copy batch metadata, and autosave-on-mutation behavior.

## Blocked by

- [013 - Introduce review document Module for row navigation and anchors](013-introduce-review-document-module.md)
