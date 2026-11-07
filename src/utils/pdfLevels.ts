// PDF compression levels (Ghostscript's, see `src-tauri/src/tauri_commands/pdf.rs`) and how the right-click menu's
// sizes pick one: Ghostscript can't aim at a size, so the levels are tried from the largest output (High) to the
// smallest (Small), and the first that fits is kept; when none fits, Small is. Kept free of imports so
// `scripts/test-pdf-levels.mjs` can run it.

/** Must match `PdfLevel` in pdf.rs */
export type PdfLevel = 'small' | 'medium' | 'high'

export const PDF_LEVELS: PdfLevel[] = ['small', 'medium', 'high']

/** Quick Compress's level unless the user chooses another (Settings -> Right-click Menu) */
export const DEFAULT_PDF_LEVEL: PdfLevel = 'medium'

/** The order the levels are tried in to reach a size: largest output first */
export const PDF_LEVELS_LARGEST_FIRST: PdfLevel[] = ['high', 'medium', 'small']

const BYTES_PER_MB = 1000 * 1000

export function isPdfLevel(value: unknown): value is PdfLevel {
  return PDF_LEVELS.includes(value as PdfLevel)
}

/** What the right-click menu asked for a PDF (the queue item's fields, see `dropQueue.ts`) */
type PdfMenuChoice = {
  targetPercent: number | null
  fitToMb: number | null
  fitToCustom: boolean
}

/**
 * The size, in bytes, a PDF of `originalSize` bytes must fit within, or `null` when the menu asked for none
 * (Quick Compress); `customFitMb` is "Your size"
 */
export function pdfTargetBytes(
  choice: PdfMenuChoice,
  originalSize: number,
  customFitMb: number,
) {
  const fitToMb = choice.fitToCustom ? customFitMb : choice.fitToMb
  if (fitToMb) {
    return fitToMb * BYTES_PER_MB
  }
  if (choice.targetPercent) {
    return (originalSize * (100 - choice.targetPercent)) / 100
  }
  return null
}

/** Whether a compressed size reaches the target */
export function fitsTarget(size: number, targetBytes: number) {
  return size <= targetBytes
}
