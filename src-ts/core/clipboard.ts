import { execFile } from "node:child_process"
import { writeFile } from "node:fs/promises"
import { tmpdir } from "node:os"
import { join } from "node:path"

export async function writeClipboard(text: string): Promise<string> {
  for (const command of systemClipboardCommands()) {
    try {
      await writeToCommand(command[0], command.slice(1), text)
      return command[0]
    } catch {}
  }
  const path = join(tmpdir(), "hawk-review-prompt.md")
  await writeFile(path, text)
  return path
}

function systemClipboardCommands(): string[][] {
  if (process.platform === "darwin") return [["pbcopy"]]
  return [["wl-copy"], ["xclip", "-selection", "clipboard"]]
}

function writeToCommand(command: string, args: string[], text: string): Promise<void> {
  return new Promise((resolve, reject) => {
    const child = execFile(command, args, (error) => error ? reject(error) : resolve())
    child.stdin?.end(text)
  })
}
