import { execFile } from "node:child_process"
import { promisify } from "node:util"
const exec = promisify(execFile)

export interface GitAdapter {
  run(repo: string, args: string[]): Promise<{ stdout: string; stderr: string }>
}

export class CliGit implements GitAdapter {
  async run(repo: string, args: string[]) {
    try {
      return await exec("git", args, { cwd: repo, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 })
    } catch (error) {
      const e = error as { stderr?: string; message?: string }
      throw new Error(e.stderr || e.message || "git failed")
    }
  }
}

export async function repoRoot(git: GitAdapter, cwd: string): Promise<string | undefined> {
  try { return (await git.run(cwd, ["rev-parse", "--show-toplevel"])).stdout.trim() } catch { return undefined }
}
export async function hasChanges(git: GitAdapter, repo: string): Promise<boolean> {
  try { return (await git.run(repo, ["status", "--porcelain"])).stdout.trim().length > 0 } catch { return true }
}
