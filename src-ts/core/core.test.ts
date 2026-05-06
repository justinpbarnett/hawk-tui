import test from "node:test"
import assert from "node:assert/strict"
import { parseUnifiedDiff } from "./diff.js"
import { ReviewDocument } from "./document.js"
import type { RepoDiff, Session } from "./model.js"
import { defaultSession } from "./session.js"
import { buildPrompt } from "./prompt.js"
import { anchorForLine, upsertComment } from "./session.js"

test("parses unified diff line numbers and rename metadata", () => {
  const files = parseUnifiedDiff("diff --git a/a.ts b/b.ts\nrename from a.ts\nrename to b.ts\n--- a/a.ts\n+++ b/b.ts\n@@ -1,2 +1,2 @@\n old\n-rm\n+add\n")
  assert.equal(files[0]?.status.kind, "renamed")
  assert.equal(files[0]?.hunks[0]?.lines[1]?.oldLine, 2)
  assert.equal(files[0]?.hunks[0]?.lines[2]?.newLine, 2)
})

test("review document skips ghost comments during changed-line navigation", () => {
  const repo = fixtureRepo()
  let session = defaultSession("/tmp/repo")
  const line = repo.files[0]!.hunks[0]!.lines[0]!
  session = upsertComment(session, anchorForLine(".", "a.ts", "@@", line), "first\nsecond")
  const doc = ReviewDocument.fromRepos([repo], session)
  assert.equal(doc.rows.some((r) => r.kind === "commentGhost"), true)
  assert.equal(doc.nextChangedLineAfter(0), 3)
})

test("prompt renders multiline comments as an indented block", () => {
  let session = defaultSession("/tmp/repo")
  const line = { kind: "add" as const, newLine: 3, text: "x" }
  session = upsertComment(session, anchorForLine(".", "a.ts", "@@ -1 +1", line), "abcd\nefg\n\n")
  const prompt = buildPrompt(session, "uncopied")!.prompt
  assert.match(prompt, /new line 3 \(added\)/)
  assert.match(prompt, /Comment:\n    abcd\n    efg\n/)
  assert.doesNotMatch(prompt, /efg\n    \n/)
})

function fixtureRepo(): RepoDiff {
  return {
    repoPath: "/tmp/repo",
    displayPath: ".",
    files: [{ oldPath: "a.ts", newPath: "a.ts", status: { kind: "modified" }, hunks: [{ header: "@@", lines: [{ kind: "add", newLine: 1, text: "x" }, { kind: "remove", oldLine: 1, text: "y" }] }] }],
  }
}
