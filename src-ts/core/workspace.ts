import { readdir } from "node:fs/promises"
import { basename, join, relative } from "node:path"
import type { Config } from "./model.js"
import type { GitAdapter } from "./git.js"
import { hasChanges, repoRoot } from "./git.js"

export interface RepoTarget { root: string; display: string }

export async function discoverRepos(git: GitAdapter, cwd: string, config: Config, forceRepo = false, forceWorkspace = false): Promise<RepoTarget[]> {
  const roots = new Set<string>()
  if (!forceWorkspace) {
    const root = await repoRoot(git, cwd)
    if (root) {
      roots.add(root)
      if (forceRepo) return targets([...roots], cwd)
    }
  }
  if (forceWorkspace || !forceRepo) for (const root of await scan(cwd, config)) roots.add(root)
  const out = []
  for (const target of targets([...roots], cwd)) if (await hasChanges(git, target.root)) out.push(target)
  return out
}

async function scan(dir: string, config: Config): Promise<string[]> {
  if (config.workspaceExcludes.includes(basename(dir))) return []
  const entries = await readdir(dir, { withFileTypes: true }).catch(() => [])
  if (entries.some((e) => e.isDirectory() && e.name === ".git")) return [dir]
  const nested = await Promise.all(entries.filter((e) => e.isDirectory()).map((e) => scan(join(dir, e.name), config)))
  return nested.flat()
}

function targets(roots: string[], cwd: string): RepoTarget[] {
  return roots.sort().map((root) => ({ root, display: relative(cwd, root) || "." }))
}
