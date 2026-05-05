# Introduce review document Module for row navigation and anchors

**Type:** AFK  
**User stories covered:** 11, 22, 23, 30, 37, 38, 55-61

## What to build

Create a deeper review document Module that owns the rendered review stream as a domain concept. The Module should concentrate row flattening, line anchors, resolved-comment visibility, hunk lookup, comment lookup, and jump/navigation helpers behind a small interface used by the review engine and TUI.

The TUI should stop re-deriving anchors by pattern-matching raw rows. Cursor movement, hunk wrapping, visible-comment jumps, and row-to-anchor lookup should be verifiable through the review document interface.

## Acceptance criteria

- [ ] Review stream flattening moves out of the review engine into a review document Module.
- [ ] The review document interface exposes row count, selected row lookup, row-to-anchor lookup, next/previous hunk navigation, and next/previous visible unresolved comment navigation.
- [ ] Resolved-comment visibility filtering lives in the review document Module.
- [ ] The TUI no longer duplicates anchor derivation or hunk/comment search logic.
- [ ] Existing navigation behavior remains unchanged: `j/k`, `J/K`, `Tab`, `n`, `p`, and `N` still work with wrapping where expected.
- [ ] Tests cover row flattening, anchor lookup, hunk wrapping, resolved visibility, and comment jumps through the review document interface.

## Blocked by

None - can start immediately.
