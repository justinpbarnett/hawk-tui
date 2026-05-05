# Hawk TUI Issue Breakdown

These issues convert `docs/prd.md` into tracer-bullet implementation slices. They are written as local markdown issue drafts instead of published tracker issues.

## Dependency order

1. [001 - Scaffold `hawk` tracer bullet](001-scaffold-hawk-tracer-bullet.md)
2. [002 - Discover single repos and nested workspaces](002-discover-repos-and-workspaces.md)
3. [003 - Load tracked and untracked diffs](003-load-tracked-and-untracked-diffs.md)
4. [004 - Render navigable unified diff TUI](004-render-navigable-unified-diff-tui.md)
5. [005 - Add autosaved inline comments](005-add-autosaved-inline-comments.md)
6. [006 - Export comments as AI prompt](006-export-comments-as-ai-prompt.md)
7. [007 - Add sidebar, comment list, and review progress](007-add-sidebar-comment-list-progress.md)
8. [008 - Reload changed diffs and reconcile comments](008-reload-and-reconcile-comments.md)
9. [009 - Add syntax and intraline highlighting](009-add-syntax-and-intraline-highlighting.md)
10. [010 - Skip non-text files and lazy-load huge diffs](010-skip-non-text-and-lazy-load-huge-diffs.md)
11. [011 - Add config, CLI modes, and tmux docs](011-config-cli-modes-and-tmux-docs.md)
12. [012 - Harden MVP with end-to-end tests](012-harden-mvp-with-e2e-tests.md)
13. [013 - Introduce review document Module for row navigation and anchors](013-introduce-review-document-module.md)
14. [014 - Add Git adapter Module for repo discovery, diff loading, and dirty checks](014-add-git-adapter-module.md)
15. [015 - Deepen diff loading Module around review-target semantics](015-deepen-diff-loading-module.md)
16. [016 - Introduce session store Module for autosave and status transitions](016-introduce-session-store-module.md)
17. [017 - Extract prompt export pipeline Module](017-extract-prompt-export-pipeline-module.md)
18. [018 - Extract TUI command handling Module](018-extract-tui-command-handling-module.md)
19. [019 - Add architecture regression tests for deepened Modules](019-add-architecture-regression-tests.md)

## Notes

- Slices are vertical where possible: each should produce demoable behavior through core + TUI + tests.
- Earlier slices intentionally use plain rendering/fallbacks where later slices add polish.
- All issues are AFK unless a human wants to revisit UX decisions before public release.
