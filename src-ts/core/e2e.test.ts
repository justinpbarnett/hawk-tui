import test from "node:test"
import assert from "node:assert/strict"
import { mkdtemp, writeFile } from "node:fs/promises"
import { tmpdir } from "node:os"
import { join } from "node:path"
import { execFileSync } from "node:child_process"
import { ReviewEngine } from "./engine.js"
import { defaultConfig, type ReviewMode } from "./model.js"

function git(repo: string, args: string[]) {
  execFileSync("git", args, { cwd: repo, stdio: "pipe" })
}
async function initRepo() {
  const repo = await mkdtemp(join(tmpdir(), "hawk-ts-"))
  git(repo, ["init"])
  git(repo, ["config", "user.email", "hawk@example.test"])
  git(repo, ["config", "user.name", "Hawk Test"])
  await writeFile(join(repo, "a.ts"), "const a = 1\n")
  git(repo, ["add", "."])
  git(repo, ["commit", "-m", "init"])
  return repo
}
async function open(repo: string, mode: ReviewMode = { kind: "default" }) {
  const engine = new ReviewEngine({ cwd: repo, mode, forceRepo: true, config: defaultConfig() })
  await engine.open()
  return engine
}

test("TypeScript engine loads tracked and untracked working-tree changes", async () => {
  const repo = await initRepo()
  await writeFile(join(repo, "a.ts"), "const a = 2\n")
  await writeFile(join(repo, "new.py"), "print('new')\n")

  const engine = await open(repo)
  const rows = engine.document.rows

  assert.ok(rows.some((row) => row.kind === "file" && row.path === "a.ts"))
  assert.ok(rows.some((row) => row.kind === "file" && row.path === "new.py"))
  assert.ok(rows.some((row) => row.kind === "line" && row.line.kind === "add" && row.line.text.includes("2")))
})

test("TypeScript engine staged mode ignores unstaged edits", async () => {
  const repo = await initRepo()
  await writeFile(join(repo, "a.ts"), "const a = 2\n")
  git(repo, ["add", "a.ts"])
  await writeFile(join(repo, "a.ts"), "const a = 3\n")

  const engine = await open(repo, { kind: "staged" })
  const texts = engine.document.rows.flatMap((row) => row.kind === "line" ? [row.line.text] : [])

  assert.ok(texts.includes("const a = 2"))
  assert.ok(!texts.includes("const a = 3"))
})
