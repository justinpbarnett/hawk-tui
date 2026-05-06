import { Box, Text, createCliRenderer } from "@opentui/core"
import { ReviewEngine } from "./core/engine.js"
import { buildView, type ViewLine } from "./ui/view.js"
import { defaultAppState, handleKey, keyName } from "./ui/state.js"
import { writePromptWithFallback } from "./ui/clipboard.js"

const renderer = await createCliRenderer({ exitOnCtrlC: false })
const engine = new ReviewEngine()
await engine.open()
let state = defaultAppState()

function render() {
  try {
    renderer.root.remove("app")
  } catch {}
  const view = buildView(engine, state)
  const rows = visibleMainRows(view.main).map((line) => Text({ content: line.text, fg: fg(line), bg: line.sourceRow === state.cursor ? "#45475a" : undefined }))
  const diff = Box({ flexGrow: view.sidebar.length ? 7 : 1, flexDirection: "column", borderStyle: "rounded", title: "hawk" }, ...rows)
  const children = [diff]
  if (view.sidebar.length) children.push(Box({ flexGrow: 3, flexDirection: "column", borderStyle: "rounded", title: view.sidebarTitle }, ...view.sidebar.map((line) => Text({ content: line.text, fg: fg(line) }))))
  renderer.root.add(Box({ id: "app", width: "100%", height: "100%", flexDirection: "column" }, Box({ flexGrow: 1, flexDirection: "row", gap: 1 }, ...children), Text({ content: view.status, fg: "#cdd6f4" })))
}

renderer.keyInput.on("keypress", async (key) => {
  state = await handleKey(state, keyName(key), engine, {
    write: (text) => writePromptWithFallback(renderer, text),
  })
  if (state.quit) renderer.destroy()
  else render()
})

render()

function visibleMainRows(lines: ViewLine[]): ViewLine[] {
  const cursorLine = lines.findIndex((line) => line.sourceRow === state.cursor)
  const height = Math.max(10, renderer.height - 3)
  const start = Math.max(0, cursorLine - Math.floor(height / 2))
  return lines.slice(start, start + height)
}
function fg(line: ViewLine): string | undefined {
  if (line.kind === "add") return "#a6e3a1"
  if (line.kind === "remove") return "#f38ba8"
  if (line.kind === "comment") return "#9399b2"
  if (line.kind === "file") return "#89b4fa"
  if (line.kind === "hunk") return "#f9e2af"
  if (line.kind === "placeholder") return "#fab387"
  return undefined
}
