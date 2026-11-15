// The brief notice a right-click menu launch shows on Linux, which has no tray icon (see
// `menu_notice.rs`): what is being compressed, then that it's done or that nothing could be, &
// it closes itself a few seconds later. The hidden main window compresses the queue meanwhile.
import { createFileRoute } from '@tanstack/react-router'
import { event } from '@tauri-apps/api'
import { useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'

import Button from '@/components/Button'
import Progress from '@/components/Progress'
import { useDropQueue } from '@/hooks/useDropQueue'
import {
  closeMenuNotice,
  openMainWindowFromNotice,
  takeSkippedNote,
} from '@/tauri/commands/menuNotice'
import { type NoticeSummary, summarize } from '@/utils/menuNotice'

export const Route = createFileRoute('/webviews/menu-notice/')({
  component: MenuNotice,
})

/** How long the last message stays before the notice closes itself */
const CLOSE_AFTER_MS = 4000
/** Closes even if no media ever arrives (e.g. a launch that failed to read its files) */
const GIVE_UP_AFTER_MS = 15000
/** Must match `NOTHING_TO_COMPRESS_EVENT` in explorer_menu.rs */
const NOTHING_TO_COMPRESS_EVENT = 'nothing-to-compress'

/** How many of the chosen files can't be compressed, once the backend says (0 until then) */
function useSkippedCount() {
  const [skipped, setSkipped] = useState(0)
  useEffect(() => {
    const take = () => {
      takeSkippedNote()
        .then((count) => {
          if (count > 0) setSkipped(count)
        })
        .catch(() => {})
    }
    take()
    const unlisten = event.listen(NOTHING_TO_COMPRESS_EVENT, take)
    return () => {
      unlisten.then((stop) => stop())
    }
  }, [])
  return skipped
}

/** Closes the notice a while after its last message, or when nothing ever arrives */
function useCloseWhenDone(isFinished: boolean) {
  useEffect(() => {
    const delay = isFinished ? CLOSE_AFTER_MS : GIVE_UP_AFTER_MS
    const timer = setTimeout(() => {
      closeMenuNotice().catch(() => {})
    }, delay)
    return () => clearTimeout(timer)
  }, [isFinished])
}

function NoticeText({
  summary,
  skipped,
}: {
  summary: NoticeSummary
  skipped: number
}) {
  const { t } = useTranslation()
  if (summary.state === 'compressing') {
    return (
      <>
        <p>{t('menuNotice.compressing', { count: summary.count })}</p>
        <Progress
          aria-label={t('menuNotice.compressing', { count: summary.count })}
          size="sm"
          value={summary.progress}
        />
      </>
    )
  }
  if (summary.state === 'done') {
    return (
      <>
        {summary.done > 0 && (
          <p>{t('menuNotice.done', { count: summary.done })}</p>
        )}
        {summary.failed > 0 && (
          <p>{t('menuNotice.failed', { count: summary.failed })}</p>
        )}
      </>
    )
  }
  if (skipped > 0) {
    return <p>{t('dropZone.nothingToCompress', { count: skipped })}</p>
  }
  return <p>{t('menuNotice.starting')}</p>
}

function MenuNotice() {
  const { t } = useTranslation()
  const summary = summarize(useDropQueue())
  const skipped = useSkippedCount()
  const isFinished =
    summary.state === 'done' || (summary.state === 'waiting' && skipped > 0)
  useCloseWhenDone(isFinished)

  return (
    <main className="flex h-screen flex-col justify-between gap-2 p-3 text-sm">
      <div className="flex flex-col gap-2" aria-live="polite">
        <NoticeText summary={summary} skipped={skipped} />
      </div>
      <div className="flex justify-end gap-2">
        <Button
          size="sm"
          onPress={() => {
            openMainWindowFromNotice().catch(() => {})
          }}
        >
          {t('menuNotice.open')}
        </Button>
        <Button
          size="sm"
          onPress={() => {
            closeMenuNotice().catch(() => {})
          }}
        >
          {t('menuNotice.close')}
        </Button>
      </div>
    </main>
  )
}
