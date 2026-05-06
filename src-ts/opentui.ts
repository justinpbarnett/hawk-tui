import { Box, Text, createCliRenderer } from "@opentui/core"
import { ReviewEngine } from "./core/engine.js"
import { buildReviewScreen, type CodeLine, type FileCard, type ViewLine } from "./ui/view.js"
import { defaultAppState, handleKey, keyName } from "./ui/state.js"
import { writePromptWithFallback } from "./ui/clipboard.js"

const theme = {
  bg: "#000000",
  panel: "#050505",
  header: "#1a1a1a",
  border: "#2a2a2a",
  text: "#d8d8d8",
  muted: "#8a8a8a",
  green: "#8cff57",
  red: "#ff6b61",
  addedBg: "#203816",
  removedBg: "#4a211f",
  selectedBg: "#2d421f",
  comment: "#22b8db",
}

const renderer = await createCliRenderer({ exitOnCtrlC: false, backgroundColor: theme.bg })
const engine = new ReviewEngine()
await engine.open()
let state = defaultAppState()

function render() {
  try {
    renderer.root.remove("app")
  } catch {}
  const screen = buildReviewScreen(engine, state)
  const main = Box(
    { flexGrow: screen.sidebar.length ? 7 : 1, flexDirection: "column", gap: 1, backgroundColor: theme.bg },
    topBar(screen),
    ...screen.cards.slice(0, visibleCardCount()).map(cardBox),
    Text({ content: screen.status, fg: theme.muted, bg: "#0b0b0b" }),
  )
  const children = [main]
  if (screen.sidebar.length) children.push(sidebarBox(screen.sidebarTitle ?? "", screen.sidebar))
  renderer.root.add(Box({ id: "app", width: "100%", height: "100%", flexDirection: "row", gap: 1, backgroundColor: theme.bg }, ...children))
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
    { height: 3, flexDirection: "row", justifyContent: "space-between", backgroundColor: theme.bg },
    Text({ content: ` ${screen.branch}  📄 ${screen.fileCount} • `, fg: theme.text }),
    Text({ content: `+${screen.added}`, fg: theme.green }),
    Text({ content: ` -${screen.removed}`, fg: theme.red }),
    Text({ content: `                 ${screen.title}                 `, fg: theme.text }),
    Text({ content: "  ↶ Discard all  📎 ", fg: theme.muted }),
  )
}

function cardBox(card: FileCard) {
  return Box(
    { flexDirection: "column", border: true, borderStyle: "rounded", borderColor: theme.border, backgroundColor: theme.panel },
    fileHeader(card),
    ...card.rows.slice(0, 18).map(codeLineBox),
    Text({ content: "    ⌄", fg: theme.muted, bg: "#0b0b0b" }),
  )
}

function fileHeader(card: FileCard) {
  return Box(
    { height: 3, flexDirection: "row", backgroundColor: theme.header },
    Text({ content: `  ⌄  ${card.path}  ⧉  `, fg: theme.text, bg: theme.header }),
    Text({ content: ` +${card.added} `, fg: theme.green, bg: "#050505" }),
    Text({ content: "•", fg: theme.muted, bg: "#050505" }),
    Text({ content: ` -${card.removed} `, fg: theme.red, bg: "#050505" }),
    Text({ content: "                                      📎  ↶  ⤴ ", fg: theme.text, bg: theme.header }),
  )
}

function codeLineBox(line: CodeLine) {
  const bg = line.kind === "add" ? theme.addedBg : line.kind === "remove" ? theme.removedBg : line.sourceRow === state.cursor ? theme.selectedBg : theme.bg
  return Box(
    { height: 1, flexDirection: "row", backgroundColor: bg },
    Text({ content: `${line.number ?? ""}`.padStart(4), fg: theme.muted, bg }),
    Text({ content: "  ", bg }),
    Text({ content: line.text, fg: fgLine(line), bg, truncate: true }),
  )
}

function sidebarBox(title: string, lines: ViewLine[]) {
  return Box(
    { flexGrow: 3, flexDirection: "column", border: true, borderStyle: "rounded", borderColor: theme.border, title, backgroundColor: theme.panel },
    ...lines.map((line) => Text({ content: line.text, fg: fgView(line), bg: theme.panel })),
  )
}

function visibleCardCount() {
  return Math.max(1, Math.floor((renderer.height - 4) / 10))
}
function fgLine(line: CodeLine): string {
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
  if (line.kind === "file") return "#89b4fa"
  return theme.text
}
