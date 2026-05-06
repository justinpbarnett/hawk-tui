import { Box, Text, createCliRenderer } from "@opentui/core"
import { ReviewEngine } from "./core/engine.js"
import { buildReviewScreen, type CodeLine, type FileCard, type ViewLine } from "./ui/view.js"
import { defaultAppState, handleKey, keyName } from "./ui/state.js"
import { writePromptWithFallback } from "./ui/clipboard.js"

const theme = {
  border: "#3a3a3a",
  text: undefined,
  muted: "#8a8a8a",
  green: "#8cff57",
  red: "#ff6b61",
  addedBg: "#203816",
  removedBg: "#4a211f",
  selectedBg: "#4b6f2a",
  comment: "#22b8db",
  file: "#89b4fa",
  hunk: "#f9e2af",
}

const renderer = await createCliRenderer({ exitOnCtrlC: false })
const engine = new ReviewEngine()
await engine.open()
let state = defaultAppState()

function render() {
  try {
    renderer.root.remove("app")
  } catch {}
  const screen = buildReviewScreen(engine, state)
  const main = Box(
    { flexGrow: screen.sidebar.length ? 7 : 1, flexDirection: "column", gap: 1 },
    topBar(screen),
    ...screen.cards.slice(0, visibleCardCount()).map(cardBox),
    Text({ content: screen.status, fg: theme.muted }),
  )
  const children = [main]
  if (screen.sidebar.length) children.push(sidebarBox(screen.sidebarTitle ?? "", screen.sidebar))
  renderer.root.add(Box({ id: "app", width: "100%", height: "100%", flexDirection: "row", gap: 1 }, ...children))
}

renderer.keyInput.on("keypress", async (key) => {
  state = await handleKey(state, keyName(key), engine, {
    write: (text) => writePromptWithFallback(renderer, text),
  })
  if (state.quit) renderer.destroy()
  else render()
})

render()

function topBar(screen: ReturnType<typeof buildReviewScreen>) {
  return Box(
    { height: 2, flexDirection: "row" },
    Text({ content: ` ${screen.branch}  📄 ${screen.fileCount} • `, fg: theme.text }),
    Text({ content: `+${screen.added}`, fg: theme.green }),
    Text({ content: ` -${screen.removed}`, fg: theme.red }),
    Text({ content: `     ${screen.title}`, fg: theme.text }),
  )
}

function cardBox(card: FileCard) {
  return Box(
    { flexDirection: "column", border: true, borderStyle: "rounded", borderColor: theme.border },
    fileHeader(card),
    ...(card.collapsed ? [Text({ content: "    ⋯ collapsed — press Enter to expand", fg: theme.muted })] : card.rows.slice(0, 18).map(codeLineBox)),
    Text({ content: card.collapsed ? "" : "    ⌄", fg: theme.muted }),
  )
}

function fileHeader(card: FileCard) {
  return Box(
    { height: 2, flexDirection: "row" },
    Text({ content: `  ${card.collapsed ? "›" : "⌄"}  ${card.path}  `, fg: theme.file }),
    Text({ content: ` +${card.added} `, fg: theme.green }),
    Text({ content: "•", fg: theme.muted }),
    Text({ content: ` -${card.removed} `, fg: theme.red }),
  )
}

function codeLineBox(line: CodeLine) {
  const selected = line.sourceRow === state.cursor
  const bg = selected ? theme.selectedBg : line.kind === "add" ? theme.addedBg : line.kind === "remove" ? theme.removedBg : undefined
  return Box(
    { height: 1, flexDirection: "row", backgroundColor: bg },
    Text({ content: selected ? "▌" : " ", fg: theme.green, bg }),
    Text({ content: `${line.number ?? ""}`.padStart(4), fg: selected ? theme.text : theme.muted, bg }),
    Text({ content: "  ", bg }),
    Text({ content: line.text, fg: selected ? theme.text : fgLine(line), bg, truncate: true }),
  )
}

function sidebarBox(title: string, lines: ViewLine[]) {
  return Box(
    { flexGrow: 3, flexDirection: "column", border: true, borderStyle: "rounded", borderColor: theme.border, title },
    ...lines.map((line) => Text({ content: line.text, fg: fgView(line) })),
  )
}

function visibleCardCount() {
  return Math.max(1, Math.floor((renderer.height - 4) / 10))
}
function fgLine(line: CodeLine): string | undefined {
  if (line.kind === "add") return theme.text
  if (line.kind === "remove") return theme.text
  if (line.kind === "comment") return theme.comment
  if (line.kind === "ellipsis") return theme.muted
  return theme.text
}
function fgView(line: ViewLine): string | undefined {
  if (line.kind === "add") return theme.green
  if (line.kind === "remove") return theme.red
  if (line.kind === "comment") return theme.comment
  if (line.kind === "file") return theme.file
  return theme.text
}
