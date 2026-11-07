// PDFs in the background queue (from the right-click menu): compressed with the bundled Ghostscript
// (`compress_pdf`, see pdf.rs) and saved like the queue's other media (`saveQueueOutput`). Quick Compress uses the
// PDF level set in Settings -> Right-click Menu; Compress by and Fit to size try the levels until one fits (see
// `utils/pdfLevels.ts`); Remove metadata keeps the highest level and empties the document information.
import { core } from '@tauri-apps/api'

import { t } from '@/i18n'
import { saveQueueOutput } from '@/routes/(root)/-save'
import { settingsStore } from '@/stores/settingsStore'
import { logError } from '@/tauri/commands/app'
import {
  type DropQueueItem,
  updateDropQueueItem,
} from '@/tauri/commands/dropQueue'
import { deleteFile, getFileMetadata } from '@/tauri/commands/fs'
import {
  fitsTarget,
  PDF_LEVELS_LARGEST_FIRST,
  type PdfLevel,
  pdfTargetBytes,
} from '@/utils/pdfLevels'
import { reportSaved } from './-queue'

type PdfOutput = { path: string; size: number }

/** Whether a queued file is a PDF (by its extension) */
export function isPdfItem(item: DropQueueItem) {
  return item.path.toLowerCase().endsWith('.pdf')
}

async function compressPdf(
  path: string,
  level: PdfLevel,
  stripMetadata: boolean,
): Promise<PdfOutput> {
  const [outputPath, size] = await core.invoke<[string, number]>(
    'compress_pdf',
    { path, level, stripMetadata },
  )
  return { path: outputPath, size }
}

/** Tries the levels from the largest output down and keeps the first that fits (or the smallest) */
async function compressPdfToFit(
  path: string,
  targetBytes: number,
): Promise<PdfOutput> {
  let output: PdfOutput | null = null
  for (const level of PDF_LEVELS_LARGEST_FIRST) {
    if (output) deleteFile(output.path).catch(() => {})
    output = await compressPdf(path, level, false)
    if (fitsTarget(output.size, targetBytes)) break
  }
  return output as PdfOutput
}

/** The PDF compressed as the right-click menu asked */
function compressForMenu(item: DropQueueItem, originalSize: number) {
  if (item.removeMetadata) {
    return compressPdf(item.path, 'high', true)
  }
  const targetBytes = pdfTargetBytes(
    item,
    originalSize,
    settingsStore.quickCompressFitSizeInMB,
  )
  if (targetBytes != null) {
    return compressPdfToFit(item.path, targetBytes)
  }
  return compressPdf(item.path, settingsStore.quickCompressPdfLevel, false)
}

/** Compresses & saves a queued PDF, reporting to the queue; returns whether it was compressed */
export async function compressPdfQueueItem(item: DropQueueItem) {
  await updateDropQueueItem(item.id, { status: 'compressing', progress: 0 })
  try {
    const metadata = await getFileMetadata(item.path)
    await updateDropQueueItem(item.id, { originalSize: metadata.size })
    const output = await compressForMenu(item, metadata.size)
    const saved = await saveQueueOutput(
      {
        sourcePath: item.path,
        relativeDir: item.relativeDir,
        originalSize: metadata.size,
        outputPath: output.path,
        outputSize: output.size,
        outputFileName: metadata.fileName,
        presetId: undefined,
        settings: item.cliJob,
      },
      { groupKey: item.groupId, topFolders: new Map() },
    )
    await reportSaved(item, saved, output)
    return true
  } catch (error) {
    logError(`[drop-queue] Could not compress the PDF ${item.path}`, error)
    await updateDropQueueItem(item.id, {
      status: 'failed',
      message: t('queue.compressFailed'),
      reason: 'compressFailed',
      error: error instanceof Error ? error.message : String(error),
    }).catch(() => {})
    return false
  }
}
