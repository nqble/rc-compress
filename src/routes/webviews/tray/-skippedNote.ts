// The tray's note when none of the files chosen in the right-click menu can be compressed (a `.txt`, a `.zip`):
// the popover opens and says so, instead of nothing happening. The backend decides when it is due
// (`note_skipped_files` in explorer_menu.rs); the window may load after that, so it also asks once when it starts.
import { core, event } from '@tauri-apps/api'
import { useEffect } from 'react'

import { toast } from '@/components/Toast'
import { t } from '@/i18n'
import { openTrayPopover } from '@/tauri/commands/tray'

/** Must match `NOTHING_TO_COMPRESS_EVENT` in explorer_menu.rs */
const NOTHING_TO_COMPRESS_EVENT = 'nothing-to-compress'

async function showSkippedNote() {
  const count = await core.invoke<number>('take_skipped_note')
  if (count === 0) {
    return
  }
  await openTrayPopover().catch(() => {})
  toast.info(t('dropZone.nothingToCompress', { count }), {
    classNames: { toast: 'scale-[0.75]' },
  })
}

export function useSkippedNote() {
  useEffect(() => {
    let unlisten: event.UnlistenFn | undefined
    let isMounted = true
    ;(async () => {
      unlisten = await event.listen(NOTHING_TO_COMPRESS_EVENT, () => {
        showSkippedNote().catch(() => {})
      })
      if (!isMounted) {
        unlisten()
        return
      }
      showSkippedNote().catch(() => {})
    })()
    return () => {
      isMounted = false
      unlisten?.()
    }
  }, [])
}
