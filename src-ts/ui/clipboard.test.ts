import test from "node:test"
import assert from "node:assert/strict"
import { writePromptWithFallback } from "./clipboard.js"

test("clipboard writer uses system clipboard first", async () => {
  let oscCalled = false
  const dest = await writePromptWithFallback(
    { copyToClipboardOSC52: () => { oscCalled = true; return true } },
    "prompt",
    async () => "pbcopy",
  )

  assert.equal(dest, "pbcopy")
  assert.equal(oscCalled, false)
})

test("clipboard writer falls back to OSC52 when system clipboard fails", async () => {
  let copied = ""
  const dest = await writePromptWithFallback(
    { copyToClipboardOSC52: (text) => { copied = text; return true } },
    "prompt",
    async () => { throw new Error("no pbcopy") },
  )

  assert.equal(dest, "osc52")
  assert.equal(copied, "prompt")
})

test("clipboard writer fails when all destinations fail", async () => {
  await assert.rejects(
    writePromptWithFallback(
      { copyToClipboardOSC52: () => false },
      "prompt",
      async () => { throw new Error("no system clipboard") },
    ),
    /clipboard export failed/,
  )
})
