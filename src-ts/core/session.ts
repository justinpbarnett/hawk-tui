import { createHash } from "node:crypto"
import { mkdir, readFile, rm, writeFile } from "node:fs/promises"
import { homedir } from "node:os"
import { dirname, join } from "node:path"
import type { Comment, DiffLine, LineAnchor, Session, Side } from "./model.js"

export function defaultSession(root: string): Session {
  return { workspaceRoot: root, cursor: 0, visitedHunks: [], comments: {}, batches: [], showResolved: false }
}

export function anchorForLine(repo: string, file: string, hunkHeader: string, line: DiffLine): LineAnchor {
  const side: Side = line.kind === "add" ? "new" : line.kind === "remove" ? "old" : "both"
  return {
    repo,
    file,
    side,
    oldLine: line.oldLine,
    newLine: line.newLine,
    hunkHeader,
    lineText: line.text,
    contextHash: sha256(`${hunkHeader}${line.oldLine ?? 0}${line.text}`),
  }
}

export const anchorKey = (anchor: LineAnchor): string => JSON.stringify(anchor)

export function upsertComment(session: Session, anchor: LineAnchor, body: string): Session {
  const key = anchorKey(anchor)
  const comments = { ...session.comments }
  if (body.trim() === "") {
    delete comments[key]
  } else {
    const existing = comments[key]
    comments[key] = { id: existing?.id ?? `c${Object.keys(comments).length + 1}`, anchor, body, status: "draft" }
  }
  return { ...session, comments }
}

export async function loadSession(root: string): Promise<Session> {
  try {
    return JSON.parse(await readFile(sessionPath(root), "utf8")) as Session
  } catch {
    return defaultSession(root)
  }
}

export async function saveSession(root: string, session: Session): Promise<void> {
  const path = sessionPath(root)
  await mkdir(dirname(path), { recursive: true })
  await writeFile(path, JSON.stringify(session, null, 2))
}

export async function resetSession(root: string): Promise<void> {
  await rm(sessionPath(root), { force: true })
}

export function visibleComments(session: Session): Comment[] {
  return Object.values(session.comments).filter((c) => session.showResolved || c.status !== "resolved")
}

export function sessionPath(root: string): string {
  return join(homedir(), ".local", "state", "hawk-tui", "sessions", `${sha256(root)}.json`)
}

export function sha256(input: string): string {
  return createHash("sha256").update(input).digest("hex")
}
