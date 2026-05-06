import type { DiffFile, DiffLine, Hunk } from "./model.js"

export function parseUnifiedDiff(input: string): DiffFile[] {
  const files: DiffFile[] = []
  let cur: DiffFile | undefined
  let hunk: Hunk | undefined
  let oldLine = 0
  let newLine = 0
  let renamedFrom: string | undefined
  let renamedTo: string | undefined

  const finishHunk = () => {
    if (hunk && cur) cur.hunks.push(hunk)
    hunk = undefined
  }
  const finishFile = () => {
    finishHunk()
    if (cur) {
      if (renamedFrom && renamedTo) cur.status = { kind: "renamed", from: renamedFrom, to: renamedTo }
      files.push(cur)
    }
    cur = undefined
    renamedFrom = undefined
    renamedTo = undefined
  }

  for (const line of input.split(/\r?\n/)) {
    if (line.startsWith("diff --git ")) {
      finishFile()
      cur = { status: { kind: "modified" }, hunks: [] }
    } else if (line.startsWith("rename from ")) {
      renamedFrom = line.slice("rename from ".length)
    } else if (line.startsWith("rename to ")) {
      renamedTo = line.slice("rename to ".length)
    } else if (line.startsWith("deleted file mode")) {
      if (cur) cur.status = { kind: "deleted" }
    } else if (line.startsWith("new file mode")) {
      if (cur) cur.status = { kind: "added" }
    } else if (line.startsWith("--- ")) {
      if (cur) cur.oldPath = line.slice(4) === "/dev/null" ? undefined : cleanPath(line.slice(4))
    } else if (line.startsWith("+++ ")) {
      if (cur) cur.newPath = line.slice(4) === "/dev/null" ? undefined : cleanPath(line.slice(4))
    } else if (line.startsWith("@@ ")) {
      finishHunk()
      const nums = parseHunkLineNumbers(line)
      oldLine = nums.oldLine
      newLine = nums.newLine
      hunk = { header: line, lines: [] }
    } else if (hunk) {
      if (line.startsWith("\\ No newline")) continue
      const prefix = line[0]
      const text = prefix === "+" || prefix === "-" || prefix === " " ? line.slice(1) : line
      let diffLine: DiffLine
      if (prefix === "+") {
        diffLine = { kind: "add", newLine, text }
        newLine++
      } else if (prefix === "-") {
        diffLine = { kind: "remove", oldLine, text }
        oldLine++
      } else {
        diffLine = { kind: "context", oldLine, newLine, text }
        oldLine++
        newLine++
      }
      hunk.lines.push(diffLine)
    }
  }
  finishFile()
  return files
}

export function newFileDiff(path: string, content: string): DiffFile {
  return {
    newPath: path,
    status: { kind: "added" },
    hunks: [
      {
        header: "@@ -0,0 +1 @@",
        lines: content.split(/\r?\n/).filter((_, i, arr) => i < arr.length - 1 || arr[i] !== "").map((text, i) => ({ kind: "add", newLine: i + 1, text })),
      },
    ],
  }
}

function cleanPath(path: string): string {
  return path.trim().replace(/^[ab]\//, "")
}

function parseHunkLineNumbers(header: string): { oldLine: number; newLine: number } {
  let oldLine = 1
  let newLine = 1
  for (const part of header.split(/\s+/)) {
    if (part.startsWith("-")) oldLine = Number(part.slice(1).split(",")[0]) || 1
    if (part.startsWith("+")) newLine = Number(part.slice(1).split(",")[0]) || 1
  }
  return { oldLine, newLine }
}
