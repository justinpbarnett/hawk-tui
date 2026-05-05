# Export comments as AI prompt

**Type:** AFK  
**User stories covered:** 4, 39, 40, 41, 42, 43, 44, 45, 46, 47, 61, 74

## What to build

Generate markdown prompts from review comments and copy them through a robust clipboard pipeline. `y` and `:w` should copy only uncopied comments. `Y` and `:w!` should copy all visible comments. Prompts should be grouped by repo and file, include side-aware references, and include compact diff context.

After successful export, included draft comments become `copied`. Export attempts with no eligible comments should be no-ops with clear status messages.

## Acceptance criteria

- [ ] `y` copies only uncopied visible comments.
- [ ] `Y` copies all visible comments.
- [ ] `:w` matches `y`; `:w!` matches `Y`.
- [ ] Zero-comment copy attempts do not mutate the clipboard or session and show a clear status message.
- [ ] Prompt output includes a concise AI-facing preamble.
- [ ] Prompt comments are grouped by repo and file.
- [ ] Each prompt item includes side-aware file/line references.
- [ ] Each prompt item includes hunk header plus compact surrounding context.
- [ ] Removed-line and deleted-file comments produce valid old-side references.
- [ ] Clipboard export tries system clipboard, then OSC52, then temp-file fallback.
- [ ] Comments are marked copied only after export succeeds or a temp file is written.
- [ ] Copy batch metadata stores timestamp, scope, comment ids, and prompt hash, but not full prompt text.
- [ ] Prompt and clipboard tests use fakes and cover selection scope, grouping, references, fallback ordering, and status mutation.

## Blocked by

- [005 - Add autosaved inline comments](005-add-autosaved-inline-comments.md)
