// What the brief notice of a right-click menu launch says (Linux, see `menu_notice.rs` and
// `routes/webviews/menu-notice/`), worked out from the queue. The whole queue is this launch's: the
// notice shows only while the main window is hidden, i.e. in an app the menu launch started (it quits
// when the notice closes), and further launches meanwhile join the same notice.

/** The queue item fields the notice reads (a `DropQueueItem`) */
export type NoticeItem = {
  id: string
  status: 'waiting' | 'compressing' | 'done' | 'failed' | 'cancelled'
  progress: number
}

export type NoticeSummary =
  | { state: 'waiting' }
  | { state: 'compressing'; count: number; progress: number }
  | { state: 'done'; done: number; failed: number }

const FINISHED = new Set(['done', 'failed', 'cancelled'])

export function summarize(items: NoticeItem[]): NoticeSummary {
  const active = items.filter((item) => !FINISHED.has(item.status))
  if (active.length > 0) {
    const total = items.reduce(
      (sum, item) => sum + (FINISHED.has(item.status) ? 100 : item.progress),
      0,
    )
    return {
      state: 'compressing',
      count: active.length,
      progress: Math.round(total / items.length),
    }
  }
  if (items.length === 0) return { state: 'waiting' }
  const done = items.filter((item) => item.status === 'done').length
  const failed = items.filter((item) => item.status === 'failed').length
  return { state: 'done', done, failed }
}
