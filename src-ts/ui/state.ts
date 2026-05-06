import type { ReviewEngine } from "../core/engine.js"

export type Mode = "nav" | "help" | "editing" | "comments"
export interface AppState { cursor: number; mode: Mode; sidebar: boolean; editBuffer: string; status: string; quit: boolean }
export const defaultAppState = (): AppState => ({ cursor: 0, mode: "nav", sidebar: false, editBuffer: "", status: "", quit: false })

export async function handleKey(state: AppState, key: string, engine: ReviewEngine): Promise<AppState> {
  const next = { ...state }
  if (next.mode === "editing") {
    if (key === "escape") {
      next.status = await engine.comment(next.cursor, next.editBuffer)
      next.editBuffer = ""
      next.mode = "nav"
    } else if (key === "backspace") next.editBuffer = next.editBuffer.slice(0, -1)
    else if (key === "enter") next.editBuffer += "\n"
    else if (key.length === 1) next.editBuffer += key
    return next
  }
  if (next.mode === "comments") {
    if (key === "c" || key === "escape") next.mode = "nav"
    return next
  }
  if (key === "j") next.cursor = engine.document.nextChangedLineAfter(next.cursor) ?? next.cursor
  if (key === "k") next.cursor = engine.document.prevChangedLineBefore(next.cursor) ?? next.cursor
  if (key === "o") { next.mode = "editing"; next.editBuffer = "" }
  if (key === "e") next.sidebar = !next.sidebar
  if (key === "c") { next.sidebar = false; next.mode = "comments" }
  if (key === "?") next.mode = next.mode === "help" ? "nav" : "help"
  if (key === "q") next.quit = true
  return next
}
