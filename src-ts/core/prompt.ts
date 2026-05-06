import { createHash } from "node:crypto"
import type { Comment, Session } from "./model.js"
import { visibleComments } from "./session.js"

export type CopyScope = "uncopied" | "all-visible"

export function buildPrompt(session: Session, scope: CopyScope): { prompt: string; ids: string[] } | undefined {
  const comments = visibleComments(session)
    .filter((c) => scope === "all-visible" || c.status === "draft")
    .sort((a, b) => `${a.anchor.repo}\0${a.anchor.file}\0${a.anchor.oldLine ?? a.anchor.newLine ?? 0}`.localeCompare(`${b.anchor.repo}\0${b.anchor.file}\0${b.anchor.oldLine ?? b.anchor.newLine ?? 0}`))
  if (comments.length === 0) return undefined

  let out = "Please address these local Hawk review comments. Keep changes focused and reply with a summary.\n"
  const ids: string[] = []
  let repo = ""
  let file = ""
  for (const comment of comments) {
    if (comment.anchor.repo !== repo) {
      repo = comment.anchor.repo
      out += `\n## Repo \`${repo}\`\n`
      file = ""
    }
    if (comment.anchor.file !== file) {
      file = comment.anchor.file
      out += `\n### \`${file}\`\n`
    }
    ids.push(comment.id)
    out += formatComment(comment)
  }
  return { prompt: out, ids }
}

export function markCopied(session: Session, ids: string[], scope: string, prompt: string): Session {
  const comments = { ...session.comments }
  for (const [key, comment] of Object.entries(comments)) {
    if (ids.includes(comment.id) && comment.status === "draft") comments[key] = { ...comment, status: "copied" }
  }
  return {
    ...session,
    comments,
    batches: [...session.batches, { id: `batch-${session.batches.length + 1}`, timestamp: new Date().toISOString(), scope, commentIds: ids, promptHash: createHash("sha256").update(prompt).digest("hex") }],
  }
}

function formatComment(comment: Comment): string {
  const line = comment.anchor.side === "old"
    ? `old line ${comment.anchor.oldLine ?? 0} (removed)`
    : comment.anchor.side === "new"
      ? `new line ${comment.anchor.newLine ?? 0} (added)`
      : `line ${comment.anchor.newLine ?? comment.anchor.oldLine ?? 0} (context)`
  return `- ${comment.anchor.file} (${line})\n  Hunk: \`${comment.anchor.hunkHeader}\`\n  Context: \`${comment.anchor.lineText}\`\n  Comment:\n${formatBody(comment.body)}\n`
}

function formatBody(body: string): string {
  return body.trimEnd().split(/\r?\n/).map((line) => `    ${line}`).join("\n")
}
