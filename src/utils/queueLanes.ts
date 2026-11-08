// How many files of each kind the background queue compresses at once (see `runDropQueue` in
// `routes/webviews/tray/-queue.ts`). Kept free of imports so `scripts/test-queue-lanes.mjs` can run it.

/** The kinds of media the queue compresses side by side, one lane each */
export type Lane = 'video' | 'audio' | 'image' | 'pdf'

/**
 * Photos compress mostly on one core: on a 16-thread PC, 50 photos took 310 s one after another at
 * 7-11 % CPU (Master, 2026-11-07). Videos and audio already use the cores (the encoder's own
 * threads), so only images run several at once, up to this many
 */
export const MAX_IMAGE_WORKERS = 4

/** Logical cores per image worker, so the rest of the computer (and a video in its lane) keeps room */
const CORES_PER_IMAGE_WORKER = 4

/** How many files of `lane` are compressed at once on a computer with `cores` logical cores */
export function laneWorkers(lane: Lane, cores: number) {
  if (lane !== 'image') {
    return 1
  }
  const workers = Math.floor(cores / CORES_PER_IMAGE_WORKER)
  return Math.min(MAX_IMAGE_WORKERS, Math.max(1, workers))
}
