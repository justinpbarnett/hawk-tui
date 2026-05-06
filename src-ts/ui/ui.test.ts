import test from "node:test"
import assert from "node:assert/strict"
import { ReviewEngine } from "../core/engine.js"
import type { RepoDiff } from "../core/model.js"
import { defaultSession, upsertComment, anchorForLine } from "../core/session.js"
import { ReviewDocument } from "../core/document.js"
import { defaultAppState, handleKey, keyName } from "./state.js"
import { buildReviewScreen, buildView } from "./view.js"

test("comment editing is inline and multiline in the view", async () => {
  const engine = fixtureEngine()
  let state = defaultAppState()
  state.cursor = 3
  state = await handleKey(state, "o", engine)
  state = await handleKey(state, "a", engine)
  state = await handleKey(state, "enter", engine)
  state = await handleKey(state, "b", engine)

  const view = buildView(engine, state)

  assert.deepEqual(view.main.filter((line) => line.kind === "comment").map((line) => line.text), ["  💬 a", "     b"])
})

test("OpenTUI key names normalize whitespace for comment editing", async () => {
  assert.equal(keyName({ name: "space", sequence: " " }), " ")
  assert.equal(keyName({ name: "return", sequence: "\r" }), "enter")
  assert.equal(keyName({ sequence: "\n" }), "enter")
})

test("comment editor accepts spaces and enter", async () => {
  const engine = fixtureEngine()
  let state = defaultAppState()
  state.cursor = 3
  state = await handleKey(state, "o", engine)
  for (const key of ["a", " ", "b", "enter", "c"]) state = await handleKey(state, key, engine)

  assert.equal(state.editBuffer, "a b\nc")
})

test("c toggles the comment sidebar and e opens a side-by-side file sidebar", async () => {
  const engine = fixtureEngine()
  let state = defaultAppState()

  state = await handleKey(state, "c", engine)
  assert.equal(state.mode, "comments")
  assert.equal(buildView(engine, state).sidebarTitle, "Comments")
  state = await handleKey(state, "c", engine)
  assert.equal(state.mode, "nav")

  state = await handleKey(state, "e", engine)
  const view = buildView(engine, state)
  assert.equal(view.sidebarTitle, "Files")
  assert.ok(view.main.some((line) => line.text.includes("+x")))
  assert.ok(view.sidebar.some((line) => line.text.includes("a.ts")))
})

test("review screen groups rows into GitHub-like file cards", () => {
  const engine = fixtureEngine()
  const screen = buildReviewScreen(engine, defaultAppState())

  assert.equal(screen.branch, "main")
  assert.equal(screen.fileCount, 1)
  assert.equal(screen.added, 1)
  assert.equal(screen.removed, 1)
  assert.equal(screen.cards[0]?.path, "a.ts")
  assert.equal(screen.cards[0]?.collapsed, false)
  assert.deepEqual(screen.cards[0]?.rows.map((row) => row.kind), ["add", "remove"])
})

test("h and l collapse and expand the current file card", async () => {
  const engine = fixtureEngine()
  let state = defaultAppState()
  state.cursor = 3

  state = await handleKey(state, "h", engine)
  let screen = buildReviewScreen(engine, state)
  assert.equal(screen.cards[0]?.collapsed, true)
  assert.deepEqual(screen.cards[0]?.rows, [])

  state = await handleKey(state, "h", engine)
  screen = buildReviewScreen(engine, state)
  assert.equal(screen.cards[0]?.collapsed, true)

  state = await handleKey(state, "l", engine)
  screen = buildReviewScreen(engine, state)
  assert.equal(screen.cards[0]?.collapsed, false)
  assert.equal(screen.cards[0]?.rows.length, 2)
})

test("enter toggles collapse for the current file card", async () => {
  const engine = fixtureEngine()
  let state = defaultAppState()
  state.cursor = 3

  state = await handleKey(state, "enter", engine)
  let screen = buildReviewScreen(engine, state)
  assert.equal(screen.cards[0]?.collapsed, true)
  assert.deepEqual(screen.cards[0]?.rows, [])

  state = await handleKey(state, "enter", engine)
  screen = buildReviewScreen(engine, state)
  assert.equal(screen.cards[0]?.collapsed, false)
  assert.equal(screen.cards[0]?.rows.length, 2)
})

test("y exports uncopied comments through the supplied writer", async () => {
  const engine = fixtureEngine()
  const line = fixtureRepo().files[0]!.hunks[0]!.lines[0]!
  engine.session = upsertComment(engine.session, anchorForLine(".", "a.ts", "@@", line), "fix")
  let copied = ""
  const state = await handleKey(defaultAppState(), "y", engine, { write: async (text) => { copied = text; return "fake" } })

  assert.match(copied, /fix/)
  assert.equal(state.status, "copied to fake")
})

function fixtureEngine(): ReviewEngine {
  const engine = new ReviewEngine()
  const repo = fixtureRepo()
  engine.repos = [repo]
  engine.session = defaultSession("/tmp/repo")
  engine.document = ReviewDocument.fromRepos([repo], engine.session)
  return engine
}

function fixtureRepo(): RepoDiff {
  return {
    repoPath: "/tmp/repo",
    displayPath: ".",
    files: [{ oldPath: "a.ts", newPath: "a.ts", status: { kind: "modified" }, hunks: [{ header: "@@", lines: [{ kind: "add", newLine: 1, text: "x" }, { kind: "remove", oldLine: 1, text: "y" }] }] }],
  }
}
