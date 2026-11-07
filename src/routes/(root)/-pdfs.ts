// PDFs added to the main window (dropped, picked, or opened with rc-compress). The window knows only
// videos, images and audio, so PDFs are handed to the background queue, which compresses them with
// the bundled Ghostscript like PDFs from the right-click menu (see `webviews/tray/-pdfQueue.ts`).
import { core } from '@tauri-apps/api'

import { toast } from '@/components/Toast'
import { t } from '@/i18n'
import { CollectedFile } from '@/types/fs'

/** The file picker's choice for PDFs */
export const PDF_PICKER_FILTER = { name: 'pdf', extensions: ['pdf'] }

function isPdfFile(file: CollectedFile) {
  return file.path.toLowerCase().endsWith('.pdf')
}

/**
 * Queues the PDFs among `files` (see `queue_pdfs` in pdf.rs) and says so; returns the other files,
 * for the window
 */
export async function handOffPdfs(files: CollectedFile[]) {
  const pdfs = files.filter(isPdfFile)
  if (pdfs.length === 0) {
    return files
  }
  const queued = await core.invoke<number>('queue_pdfs', { files: pdfs })
  if (queued > 0) {
    toast.info(t('media.pdfsQueued', { count: queued }))
  } else {
    toast.info(t('media.alreadyAdded'))
  }
  return files.filter((file) => !isPdfFile(file))
}
