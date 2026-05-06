import { cwd } from "node:process"
import type { Config, RepoDiff, ReviewMode, Session } from "./model.js"
import { defaultConfig } from "./model.js"
import { CliGit, type GitAdapter } from "./git.js"
import { discoverRepos } from "./workspace.js"
import { loadRepoDiff } from "./loader.js"
import { ReviewDocument } from "./document.js"
import { anchorKey, loadSession, saveSession, upsertComment } from "./session.js"
import { buildPrompt, markCopied, type CopyScope } from "./prompt.js"

export class ReviewEngine {
  repos: RepoDiff[] = []
  document = new ReviewDocument([])
  session!: Session

  constructor(readonly options: { cwd?: string; config?: Config; mode?: ReviewMode; forceRepo?: boolean; forceWorkspace?: boolean; git?: GitAdapter } = {}) {}

  get root() { return this.options.cwd ?? cwd() }
  get config() { return this.options.config ?? defaultConfig() }
  get mode() { return this.options.mode ?? { kind: "default" as const } }
  get git() { return this.options.git ?? new CliGit() }

  async open() {
    this.session = await loadSession(this.root)
    await this.reload()
  }

  async reload() {
    const targets = await discoverRepos(this.git, this.root, this.config, this.options.forceRepo, this.options.forceWorkspace)
    this.repos = await Promise.all(targets.map((t) => loadRepoDiff(this.git, t.root, t.display, this.mode, this.config)))
    this.document = ReviewDocument.fromRepos(this.repos, this.session)
  }

  async comment(row: number, body: string) {
    const anchor = this.document.anchorAt(row)
    if (!anchor) return "comments attach only to diff content lines"
    this.session = upsertComment(this.session, anchor, body)
    await saveSession(this.root, this.session)
    this.document = ReviewDocument.fromRepos(this.repos, this.session)
    return "comment saved"
  }

  async export(scope: CopyScope, write: (text: string) => Promise<string>) {
    const built = buildPrompt(this.session, scope)
    if (!built) return "no comments to copy"
    const dest = await write(built.prompt)
    this.session = markCopied(this.session, built.ids, scope, built.prompt)
    await saveSession(this.root, this.session)
    return `copied to ${dest}`
  }

  deleteComment(row: number) {
    const anchor = this.document.anchorAt(row)
    if (!anchor) return
    const comments = { ...this.session.comments }
    delete comments[anchorKey(anchor)]
    this.session = { ...this.session, comments }
    this.document = ReviewDocument.fromRepos(this.repos, this.session)
  }
}
