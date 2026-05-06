import { writeClipboard } from "../core/clipboard.js"
import type { ReviewEngine } from "../core/engine.js"
import { anchorKey } from "../core/session.js"

export type Mode = "nav" | "help" | "editing" | "comments"
export interface AppState {
  cursor: number
  mode: Mode
  sidebar: boolean
  editBuffer: string
  status: string
  quit: boolean
  collapsedFiles: string[]
}
export interface KeyHandlingOptions {
  write?: (text: string) => Promise<string>
}

export const defaultAppState = (): AppState => ({ cursor: 0, mode: "nav", sidebar: false, editBuffer: "", status: "", quit: false, collapsedFiles: [] })

export async function handleKey(state: AppState, key: string, engine: ReviewEngine, options: KeyHandlingOptions = {}): Promise<AppState> {
  const next = { ...state }
  if (next.mode === "editing") return handleEditingKey(next, key, engine)
  if (next.mode === "comments") return handleCommentListKey(next, key)
  if (next.mode === "help") {
    if (key === "?" || key === "escape") next.mode = "nav"
    return next
  }

  switch (key) {
    case "j": next.cursor = engine.document.nextChangedLineAfter(next.cursor) ?? next.cursor; break
    case "k": next.cursor = engine.document.prevChangedLineBefore(next.cursor) ?? next.cursor; break
    case "J":
    case "tab": next.cursor = engine.document.nextHunkAfter(next.cursor) ?? next.cursor; break
    case "K": next.cursor = prevHunk(engine, next.cursor) ?? next.cursor; break
    case "o": startEditing(next, engine); break
    case "enter": toggleCurrentFile(next, engine); break
    case "e": next.sidebar = !next.sidebar; next.mode = "nav"; break
    case "c": next.sidebar = false; next.mode = "comments"; break
    case "?": next.mode = "help"; break
    case "r": await engine.reload(); next.status = "reloaded"; break
    case "y": next.status = await engine.export("uncopied", options.write ?? writeClipboard); break
    case "Y": next.status = await engine.export("all-visible", options.write ?? writeClipboard); break
    case "x": engine.deleteComment(next.cursor); next.status = "comment deleted"; break
    case "q": next.quit = true; break
  }
  return next
}

async function handleEditingKey(state: AppState, key: string, engine: ReviewEngine): Promise<AppState> {
  const next = { ...state }
  if (key === "escape" || key === "ctrl-c") {
    next.status = await engine.comment(next.cursor, next.editBuffer)
    next.editBuffer = ""
    next.mode = "nav"
  } else if (key === "backspace") {
    next.editBuffer = next.editBuffer.slice(0, -1)
  } else if (key === "enter") {
    next.editBuffer += "\n"
  } else if (key.length === 1) {
    next.editBuffer += key
  }
  return next
}

function handleCommentListKey(state: AppState, key: string): AppState {
  const next = { ...state }
  if (key === "c" || key === "escape") next.mode = "nav"
  return next
}

function startEditing(state: AppState, engine: ReviewEngine) {
  const anchor = engine.document.anchorAt(state.cursor)
  if (!anchor) {
    state.status = "comments attach only to diff content lines"
    return
  }
  state.editBuffer = engine.session.comments[anchorKey(anchor)]?.body ?? ""
  state.mode = "editing"
}

function toggleCurrentFile(state: AppState, engine: ReviewEngine) {
  const file = currentFile(engine, state.cursor)
  if (!file) return
  state.collapsedFiles = state.collapsedFiles.includes(file)
    ? state.collapsedFiles.filter((path) => path !== file)
    : [...state.collapsedFiles, file]
}

function currentFile(engine: ReviewEngine, cursor: number): string | undefined {
  for (let i = cursor; i >= 0; i--) {
    const row = engine.document.rows[i]
    if (row?.kind === "file") return row.path
    if (row?.kind === "line" || row?.kind === "hunk") return row.file
  }
  return undefined
}

function prevHunk(engine: ReviewEngine, cursor: number): number | undefined {
  const indices = engine.document.rows.flatMap((row, i) => row.kind === "hunk" ? [i] : [])
  const before = indices.filter((i) => i < cursor)
  return before.at(-1) ?? indices.at(-1)
}

export function keyName(key: { name?: string; sequence?: string; ctrl?: boolean }): string {
  if (key.ctrl && key.name === "c") return "ctrl-c"
  return key.name ?? key.sequence ?? ""
}
