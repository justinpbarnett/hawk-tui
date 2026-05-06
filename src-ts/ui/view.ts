import type { ReviewEngine } from "../core/engine.js"
import type { ReviewRow } from "../core/document.js"
import type { AppState } from "./state.js"

export interface ViewLine { text: string; kind: "repo" | "file" | "hunk" | "add" | "remove" | "context" | "comment" | "placeholder" | "status"; sourceRow?: number }
export interface ViewModel { main: ViewLine[]; sidebarTitle?: string; sidebar: ViewLine[]; status: string }

export function buildView(engine: ReviewEngine, state: AppState): ViewModel {
  const main = state.mode === "help" ? helpLines() : diffLines(engine, state)
  const sidebar = state.sidebar ? fileSidebar(engine) : state.mode === "comments" ? commentSidebar(engine) : []
  const sidebarTitle = state.sidebar ? "Files" : state.mode === "comments" ? "Comments" : undefined
  return { main, sidebarTitle, sidebar, status: `${engine.document.rows.length} rows | ${state.status}` }
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
    "o edit inline comment, Esc saves",
    "e toggle file sidebar, c toggle comments sidebar",
    "y copy uncopied comments, Y copy all visible comments",
    "r reload, q quit, ? close help",
  ].map((text) => ({ text, kind: "status" }))
}
