import type { ReviewEngine } from "../core/engine.js"
import type { ReviewRow } from "../core/document.js"
import type { AppState } from "./state.js"

export interface ViewLine { text: string; kind: "repo" | "file" | "hunk" | "add" | "remove" | "context" | "comment" | "placeholder" | "status"; sourceRow?: number }
export interface ViewModel { main: ViewLine[]; sidebarTitle?: string; sidebar: ViewLine[]; status: string }
export interface ReviewScreen { branch: string; fileCount: number; added: number; removed: number; title: string; cards: FileCard[]; sidebarTitle?: string; sidebar: ViewLine[]; status: string }
export interface FileCard { path: string; added: number; removed: number; collapsed: boolean; rows: CodeLine[] }
export interface CodeLine { number?: number; text: string; kind: "add" | "remove" | "context" | "comment" | "ellipsis"; sourceRow?: number }

export function buildView(engine: ReviewEngine, state: AppState): ViewModel {
  const main = state.mode === "help" ? helpLines() : diffLines(engine, state)
  const sidebar = state.sidebar ? fileSidebar(engine) : state.mode === "comments" ? commentSidebar(engine) : []
  const sidebarTitle = state.sidebar ? "Files" : state.mode === "comments" ? "Comments" : undefined
  return { main, sidebarTitle, sidebar, status: `${engine.document.rows.length} rows | ${state.status}` }
}

export function buildReviewScreen(engine: ReviewEngine, state: AppState): ReviewScreen {
  const cards = fileCards(engine, state)
  const totals = cards.reduce((acc, card) => ({ added: acc.added + card.added, removed: acc.removed + card.removed }), { added: 0, removed: 0 })
  const sidebar = state.sidebar ? fileSidebar(engine) : state.mode === "comments" ? commentSidebar(engine) : []
  const sidebarTitle = state.sidebar ? "Files" : state.mode === "comments" ? "Comments" : undefined
  return {
    branch: "main",
    title: "↔  Uncommitted changes",
    fileCount: cards.length,
    added: totals.added,
    removed: totals.removed,
    cards,
    sidebar,
    sidebarTitle,
    status: state.status || "j/k changed lines • o comment • y copy • ? help",
  }
}

function fileCards(engine: ReviewEngine, state: AppState): FileCard[] {
  const cards: FileCard[] = []
  let current: FileCard | undefined
  for (const [index, row] of engine.document.rows.entries()) {
    if (row.kind === "file") {
      current = { path: row.path, added: row.added, removed: row.removed, collapsed: state.collapsedFiles.includes(row.path), rows: [] }
      cards.push(current)
    } else if (current?.collapsed) {
      continue
    } else if (current && row.kind === "line") {
      current.rows.push({
        number: row.line.kind === "remove" ? row.line.oldLine : row.line.newLine ?? row.line.oldLine,
        text: row.line.text,
        kind: row.line.kind,
        sourceRow: index,
      })
      if (state.mode === "editing" && index === state.cursor) {
        current.rows.push(...commentCodeLines(state.editBuffer || "Type comment here. Esc saves."))
      }
    } else if (current && row.kind === "commentGhost") {
      current.rows.push(...commentCodeLines(row.body))
    } else if (current && row.kind === "placeholder") {
      current.rows.push({ text: row.text, kind: "ellipsis" })
    }
  }
  return cards
}

function commentCodeLines(body: string): CodeLine[] {
  return body.split(/\r?\n/).map((line, i) => ({ text: `${i === 0 ? "💬 " : "   "}${line}`, kind: "comment" }))
}

function diffLines(engine: ReviewEngine, state: AppState): ViewLine[] {
  const lines: ViewLine[] = []
  engine.document.rows.forEach((row, index) => {
    lines.push(...rowToLines(row).map((line) => ({ ...line, sourceRow: index })))
    if (state.mode === "editing" && index === state.cursor) lines.push(...commentLines(state.editBuffer || "Type comment here. Esc saves.", "comment"))
  })
  return lines
}

function rowToLines(row: ReviewRow): ViewLine[] {
  if (row.kind === "repo") return [{ text: `repo ${row.repo}`, kind: "repo" }]
  if (row.kind === "file") return [{ text: `${row.path} +${row.added} -${row.removed}`, kind: "file" }]
  if (row.kind === "hunk") return [{ text: row.header, kind: "hunk" }]
  if (row.kind === "line") return [{ text: `${row.line.kind === "add" ? "+" : row.line.kind === "remove" ? "-" : " "}${row.line.text}`, kind: row.line.kind }]
  if (row.kind === "commentGhost") return commentLines(row.body, "comment")
  return [{ text: `! ${row.text}`, kind: "placeholder" }]
}

function commentLines(body: string, kind: ViewLine["kind"]): ViewLine[] {
  return body.split(/\r?\n/).map((line, i) => ({ text: `${i === 0 ? "  💬 " : "     "}${line}`, kind }))
}

function fileSidebar(engine: ReviewEngine): ViewLine[] {
  const files = engine.document.rows.filter((r) => r.kind === "file")
  return files.length ? files.map((row) => rowToLines(row)[0]!) : [{ text: "No changed files", kind: "status" }]
}

function commentSidebar(engine: ReviewEngine): ViewLine[] {
  const comments = Object.values(engine.session.comments)
  return comments.length ? comments.map((c) => ({ text: `${c.anchor.file}:${c.anchor.newLine ?? c.anchor.oldLine ?? 0} ${c.body.replace(/\s+/g, " ")}`, kind: "comment" })) : [{ text: "No visible comments", kind: "status" }]
}

function helpLines(): ViewLine[] {
  return [
    "Hawk help",
    "",
    "j/k changed line navigation",
    "J/K or Tab hunk navigation",
    "Enter expand/collapse current file",
    "o edit inline comment, Esc saves",
    "e toggle file sidebar, c toggle comments sidebar",
    "y copy uncopied comments, Y copy all visible comments",
    "r reload, q quit, ? close help",
  ].map((text) => ({ text, kind: "status" }))
}
