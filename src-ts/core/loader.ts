import { lstat, readFile } from "node:fs/promises"
import { join } from "node:path"
import type { Config, DiffFile, RepoDiff, ReviewMode } from "./model.js"
import { fileCounts } from "./model.js"
import { newFileDiff, parseUnifiedDiff } from "./diff.js"
import type { GitAdapter } from "./git.js"

export async function loadRepoDiff(git: GitAdapter, repo: string, display: string, mode: ReviewMode, config: Config): Promise<RepoDiff> {
  const files: DiffFile[] = []
  try {
    files.push(...parseUnifiedDiff((await git.run(repo, diffArgs(mode, config))).stdout))
  } catch (error) {
    return { repoPath: repo, displayPath: display, files, error: String(error instanceof Error ? error.message : error) }
  }
  if (mode.kind === "default" && config.includeUntracked) {
    try {
      const out = (await git.run(repo, ["ls-files", "--others", "--exclude-standard", "-z"])).stdout
      for (const rel of out.split("\0").filter(Boolean)) files.push(await untrackedDiff(repo, rel, config))
    } catch {}
  }
  return { repoPath: repo, displayPath: display, files }
}

function diffArgs(mode: ReviewMode, config: Config): string[] {
  const context = `-U${config.diffContextLines}`
  if (mode.kind === "staged") return ["diff", "--cached", "--find-renames", context]
  if (mode.kind === "base") return ["diff", "--find-renames", `${mode.base}...HEAD`, context]
  if (mode.kind === "ref") return ["diff", "--find-renames", mode.ref, context]
  return ["diff", "--find-renames", context]
}

async function untrackedDiff(repo: string, rel: string, config: Config): Promise<DiffFile> {
  const path = join(repo, rel)
  const stat = await lstat(path)
  const nonText = /\.(png|jpe?g|gif|webp|zip|gz|tar|woff|ttf|sqlite|db|pdf|mp4|mov|ico)$/i
  if (nonText.test(rel)) return { newPath: rel, status: { kind: "skipped", reason: "known non-text extension" }, hunks: [] }
  if (stat.size > config.absoluteFileSize) return { newPath: rel, status: { kind: "skipped", reason: "over absolute size limit" }, hunks: [] }
  const content = await readFile(path)
  if (content.includes(0)) return { newPath: rel, status: { kind: "skipped", reason: "contains NUL byte" }, hunks: [] }
  if (stat.size > config.eagerFileSize) return { newPath: rel, status: { kind: "collapsed", reason: "over eager file-size threshold", added: 0, removed: 0 }, hunks: [] }
  const diff = newFileDiff(rel, content.toString("utf8"))
  const counts = fileCounts(diff)
  if (counts.added + counts.removed > config.eagerDiffLines) return { newPath: rel, status: { kind: "collapsed", reason: "over eager diff-line threshold", added: counts.added, removed: counts.removed }, hunks: [] }
  return diff
}
