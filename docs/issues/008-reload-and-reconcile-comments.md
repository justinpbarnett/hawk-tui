# Reload changed diffs and reconcile comments

**Type:** AFK  
**User stories covered:** 48, 49, 50, 51, 52, 53, 54, 55, 56, 65

## What to build

Add live change detection, manual reload, and comment reconciliation. File changes should set a dirty indicator without automatically changing the visible diff. Pressing `r` reloads the diff and reconciles comments against the new document.

Copied comments whose target hunk or line changed/disappeared are auto-marked resolved and hidden by default. Draft comments remain visible. Uncertain mappings become stale. Resolved history remains recoverable.

## Acceptance criteria

- [ ] Hawk watches relevant workspace/repo file changes and sets a dirty indicator.
- [ ] Live file changes do not automatically reload or move the cursor.
- [ ] Pressing `r` reloads repo/workspace diffs.
- [ ] Reload preserves cursor as well as possible and reports a concise summary.
- [ ] Copied comments are marked resolved when their target hunk/line changed or disappeared.
- [ ] Auto-resolved comments are hidden by default but preserved in session history.
- [ ] Draft comments remain visible across reloads.
- [ ] Comments that cannot be mapped safely are marked stale and remain visible.
- [ ] `:reload` matches `r`.
- [ ] `:clear-resolved` removes resolved history after confirmation if implemented in this slice.
- [ ] Reconciliation tests cover copied-to-resolved, draft preservation, stale mappings, hidden resolved visibility, and autosave after reload.

## Blocked by

- [006 - Export comments as AI prompt](006-export-comments-as-ai-prompt.md)
- [007 - Add sidebar, comment list, and review progress](007-add-sidebar-comment-list-progress.md)
