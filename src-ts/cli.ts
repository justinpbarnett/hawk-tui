#!/usr/bin/env node
import { parseArgs } from "node:util"
import { ReviewEngine } from "./core/engine.js"
import type { ReviewMode } from "./core/model.js"

process.stdout.on("error", (error: NodeJS.ErrnoException) => {
  if (error.code === "EPIPE") process.exit(0)
  throw error
})

const { values } = parseArgs({
  options: {
    "no-tui": { type: "boolean", default: false },
    staged: { type: "boolean", default: false },
    base: { type: "string" },
    ref: { type: "string" },
    repo: { type: "boolean", default: false },
    workspace: { type: "boolean", default: false },
  },
})

const mode: ReviewMode = values.staged ? { kind: "staged" } : values.base ? { kind: "base", base: values.base } : values.ref ? { kind: "ref", ref: values.ref } : { kind: "default" }
const engine = new ReviewEngine({ mode, forceRepo: values.repo, forceWorkspace: values.workspace })
await engine.open()

if (!values["no-tui"]) {
  console.error("Interactive TypeScript TUI lives in src-ts/opentui.ts and requires Bun: bun src-ts/opentui.ts")
}
for (const row of engine.document.rows) process.stdout.write(`${JSON.stringify(row)}\n`)
