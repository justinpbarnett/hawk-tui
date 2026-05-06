import { writeClipboard } from "../core/clipboard.js"

export interface Osc52Renderer {
  copyToClipboardOSC52(text: string): boolean
}

export async function writePromptWithFallback(
  renderer: Osc52Renderer,
  text: string,
  systemWrite: (text: string) => Promise<string> = writeClipboard,
): Promise<string> {
  try {
    return await systemWrite(text)
  } catch {}

  try {
    if (renderer.copyToClipboardOSC52(text)) return "osc52"
  } catch {}

  throw new Error("clipboard export failed")
}
