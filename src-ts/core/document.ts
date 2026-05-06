import type { DiffLine, RepoDiff, Session } from "./model.js"
import { anchorForLine, anchorKey } from "./session.js"
import { fileCounts, filePath } from "./model.js"

export type ReviewRow =
  | { kind: "repo"; repo: string }
  | { kind: "file"; repo: string; path: string; added: number; removed: number }
  | { kind: "hunk"; repo: string; file: string; header: string }
  | { kind: "line"; repo: string; file: string; hunk: string; line: DiffLine }
  | { kind: "commentGhost"; body: string }
  | { kind: "placeholder"; text: string }

export class ReviewDocument {
  constructor(readonly rows: ReviewRow[]) {}

  static fromRepos(repos: RepoDiff[], session: Session): ReviewDocument {
    const rows: ReviewRow[] = []
    for (const repo of repos) {
      rows.push({ kind: "repo", repo: repo.displayPath })
      if (repo.error) {
        rows.push({ kind: "placeholder", text: `repo error: ${repo.error}` })
        continue
      }
      for (const file of repo.files) {
        const path = filePath(file)
        const counts = fileCounts(file)
        rows.push({ kind: "file", repo: repo.displayPath, path, added: counts.added, removed: counts.removed })
        if (file.status.kind === "skipped" || file.status.kind === "collapsed") {
          rows.push({ kind: "placeholder", text: file.status.reason })
          continue
        }
        for (const hunk of file.hunks) {
          rows.push({ kind: "hunk", repo: repo.displayPath, file: path, header: hunk.header })
          for (const line of hunk.lines) {
            const anchor = anchorForLine(repo.displayPath, path, hunk.header, line)
            const comment = session.comments[anchorKey(anchor)]
            if (session.showResolved || comment?.status !== "resolved") {
              rows.push({ kind: "line", repo: repo.displayPath, file: path, hunk: hunk.header, line })
              if (comment && (session.showResolved || comment.status !== "resolved")) rows.push({ kind: "commentGhost", body: comment.body })
            }
          }
        }
      }
    }
    return new ReviewDocument(rows)
  }

  anchorAt(index: number) {
    const row = this.rows[index]
    return row?.kind === "line" ? anchorForLine(row.repo, row.file, row.hunk, row.line) : undefined
  }

  nextChangedLineAfter(cursor: number, collapsedFiles: string[] = []): number | undefined {
    return wrapFind(this.navigableLines(collapsedFiles), (i) => i > cursor)
  }
  prevChangedLineBefore(cursor: number, collapsedFiles: string[] = []): number | undefined {
    const lines = this.navigableLines(collapsedFiles).reverse()
    return lines.find((i) => i < cursor) ?? lines.at(-1)
  }
  nextHunkAfter(cursor: number): number | undefined {
    return wrapFind(this.hunks(), (i) => i > cursor)
  }
  prevHunkBefore(cursor: number): number | undefined {
    const hunks = this.hunks().reverse()
    return hunks.find((i) => i < cursor) ?? hunks.at(-1)
  }

  private navigableLines(collapsedFiles: string[]): number[] {
    return this.rows.flatMap((row, i) => {
      if (row.kind === "file" && collapsedFiles.includes(row.path)) return [i]
      if (row.kind === "line" && !collapsedFiles.includes(row.file) && (row.line.kind === "add" || row.line.kind === "remove")) return [i]
      return []
    })
  }
  private hunks(): number[] {
    return this.rows.flatMap((row, i) => row.kind === "hunk" ? [i] : [])
  }
}

function wrapFind(values: number[], predicate: (value: number) => boolean): number | undefined {
  return values.find(predicate) ?? values[0]
}
