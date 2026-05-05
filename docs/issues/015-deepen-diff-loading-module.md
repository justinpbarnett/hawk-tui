# Deepen diff loading Module around review-target semantics

**Type:** AFK  
**User stories covered:** 5-8, 18-21, 70, 72-73

## What to build

Refactor diff loading into a deeper Module whose interface represents Hawk review-target semantics rather than Git command construction details. Callers should ask for a repo diff for a review mode and config; the implementation should own tracked diffs, staged/base/ref modes, untracked inclusion, ignored-file exclusion, text detection, skipped-file placeholders, huge-file collapse decisions, and parser invocation.

This should preserve the current unified diff model while improving locality for future changes to thresholds, untracked behavior, binary detection, and lazy loading.

## Acceptance criteria

- [ ] Review-target semantics are concentrated in a diff loading Module with a small caller-facing interface.
- [ ] Git command details are hidden behind the Git adapter Module.
- [ ] Default mode still includes tracked working-tree changes plus untracked non-ignored text files.
- [ ] `--staged`, `--base`, and `--ref` behavior remains unchanged.
- [ ] Non-text files still become skipped placeholders with reasons.
- [ ] Huge text/code files still become collapsed placeholders according to config thresholds.
- [ ] Parser tests remain focused on unified diff parsing, separate from diff loading behavior.
- [ ] Diff loading tests cover default, staged, base/ref error handling, untracked inclusion, ignored exclusion, skipped files, and collapsed files.

## Blocked by

- [014 - Add Git adapter Module for repo discovery, diff loading, and dirty checks](014-add-git-adapter-module.md)
