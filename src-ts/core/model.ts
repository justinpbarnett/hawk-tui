export type LineKind = "add" | "remove" | "context"
export type Side = "old" | "new" | "both"

export interface DiffLine {
  kind: LineKind
  oldLine?: number
  newLine?: number
  text: string
}

export interface Hunk {
  header: string
  lines: DiffLine[]
}

export type FileStatus =
  | { kind: "modified" }
  | { kind: "added" }
  | { kind: "deleted" }
  | { kind: "renamed"; from: string; to: string }
  | { kind: "skipped"; reason: string }
  | { kind: "collapsed"; reason: string; added: number; removed: number }

export interface DiffFile {
  oldPath?: string
  newPath?: string
  status: FileStatus
  hunks: Hunk[]
}

export interface RepoDiff {
  repoPath: string
  displayPath: string
  files: DiffFile[]
  error?: string
}

export interface LineAnchor {
  repo: string
  file: string
  side: Side
  oldLine?: number
  newLine?: number
  hunkHeader: string
  lineText: string
  contextHash: string
}

export type CommentStatus = "draft" | "copied" | "resolved" | "stale"

export interface Comment {
  id: string
  anchor: LineAnchor
  body: string
  status: CommentStatus
}

export interface CopyBatch {
  id: string
  timestamp: string
  scope: string
  commentIds: string[]
  promptHash: string
}

export interface Session {
  workspaceRoot: string
  cursor: number
  visitedHunks: string[]
  comments: Record<string, Comment>
  batches: CopyBatch[]
  showResolved: boolean
}

export type ReviewMode =
  | { kind: "default" }
  | { kind: "staged" }
  | { kind: "base"; base: string }
  | { kind: "ref"; ref: string }

export interface Config {
  diffContextLines: number
  eagerFileSize: number
  eagerDiffLines: number
  absoluteFileSize: number
  includeUntracked: boolean
  workspaceExcludes: string[]
}

export const defaultConfig = (): Config => ({
  diffContextLines: 3,
  eagerFileSize: 512 * 1024,
  eagerDiffLines: 3000,
  absoluteFileSize: 5 * 1024 * 1024,
  includeUntracked: true,
  workspaceExcludes: ["node_modules", "target", ".git", "dist", "build", "vendor"],
})

export const filePath = (file: DiffFile): string => file.newPath ?? file.oldPath ?? ""
export const fileCounts = (file: DiffFile): { added: number; removed: number } =>
  file.hunks.flatMap((h) => h.lines).reduce(
    (acc, line) => {
      if (line.kind === "add") acc.added++
      if (line.kind === "remove") acc.removed++
      return acc
    },
    { added: 0, removed: 0 },
  )
