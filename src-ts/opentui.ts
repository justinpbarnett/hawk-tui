import { Box, Text, createCliRenderer } from "@opentui/core"
import { ReviewEngine } from "./core/engine.js"
import { defaultAppState, handleKey } from "./ui/state.js"

const renderer = await createCliRenderer({ exitOnCtrlC: true })
const engine = new ReviewEngine()
await engine.open()
let state = defaultAppState()

function render() {
  try {
    renderer.root.remove("app")
  } catch {}
  const rows = engine.document.rows.map((row, i) =>
    Text({ content: rowText(row), fg: i === state.cursor ? "#000000" : color(row), bg: i === state.cursor ? "#f5c2e7" : undefined }),
  )
  const diff = Box({ flexGrow: state.sidebar || state.mode === "comments" ? 7 : 1, flexDirection: "column", borderStyle: "rounded", title: "hawk" }, ...rows)
  const children = [diff]
  if (state.sidebar) {
    children.push(Box({ flexGrow: 3, flexDirection: "column", borderStyle: "rounded", title: "Files" }, ...engine.document.rows.filter((r) => r.kind === "file").map((r) => Text({ content: rowText(r), fg: "#89b4fa" }))))
  }
  if (state.mode === "comments") {
    children.push(Box({ flexGrow: 3, flexDirection: "column", borderStyle: "rounded", title: "Comments" }, ...Object.values(engine.session.comments).map((c) => Text({ content: `${c.anchor.file}:${c.anchor.newLine ?? c.anchor.oldLine} ${c.body}`, fg: "#a6e3a1" }))))
  }
  renderer.root.add(Box({ id: "app", width: "100%", height: "100%", flexDirection: "row", gap: 1 }, ...children))
}

renderer.keyInput.on("keypress", async (key: { name?: string; sequence?: string }) => {
  state = await handleKey(state, key.name ?? key.sequence ?? "", engine)
  if (state.quit) renderer.destroy()
  render()
})

render()

function rowText(row: (typeof engine.document.rows)[number]): string {
  if (row.kind === "repo") return `repo ${row.repo}`
  if (row.kind === "file") return `${row.path} +${row.added} -${row.removed}`
  if (row.kind === "hunk") return row.header
  if (row.kind === "line") return `${row.line.kind === "add" ? "+" : row.line.kind === "remove" ? "-" : " "}${row.line.text}`
  if (row.kind === "commentGhost") return `  💬 ${row.body.replace(/\n/g, "\n     ")}`
  return `! ${row.text}`
}
function color(row: (typeof engine.document.rows)[number]): string | undefined {
  if (row.kind === "line" && row.line.kind === "add") return "#a6e3a1"
  if (row.kind === "line" && row.line.kind === "remove") return "#f38ba8"
  if (row.kind === "commentGhost") return "#9399b2"
  return undefined
}
