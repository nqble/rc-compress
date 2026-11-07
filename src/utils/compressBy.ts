// "Compress by N%" in the right-click menu (see `explorer_menu.rs`): the target size that makes a file N% smaller, in
// the units compressO's target-size compression takes (MB for videos and audio, see `targetSize.ts`; KB for images,
// see `imageTargetSize.ts`; both counted in thousands). Kept free of imports so `scripts/test-compress-by.mjs` can run
// it; the smallest sizes must match MIN_TARGET_SIZE_IN_MB and MIN_IMAGE_TARGET_SIZE_IN_KB there.

const BYTES_PER_KB = 1000
const BYTES_PER_MB = 1000 * 1000
/** Must match MIN_TARGET_SIZE_IN_MB (targetSize.ts) */
const MIN_TARGET_SIZE_IN_MB = 1
/** Must match MIN_IMAGE_TARGET_SIZE_IN_KB (imageTargetSize.ts) */
const MIN_IMAGE_TARGET_SIZE_IN_KB = 10

/** The percentages the menu offers (10% to 90%, in steps of 10); must match `explorer_menu.rs` */
export const COMPRESS_BY_PERCENTS = [10, 20, 30, 40, 50, 60, 70, 80, 90]

/** What is left of a file `percent` smaller, in bytes */
function bytesLeft(sizeInBytes: number, percent: number) {
  return (sizeInBytes * (100 - percent)) / 100
}

/** The target size of a video or audio file `percent` smaller, in MB (never below the smallest target) */
export function videoTargetSizeInMB(sizeInBytes: number, percent: number) {
  const megabytes = bytesLeft(sizeInBytes, percent) / BYTES_PER_MB
  return Math.max(MIN_TARGET_SIZE_IN_MB, Math.floor(megabytes * 100) / 100)
}

/** The target size of an image `percent` smaller, in KB (never below the smallest target) */
export function imageTargetSizeInKB(sizeInBytes: number, percent: number) {
  const kilobytes = Math.floor(bytesLeft(sizeInBytes, percent) / BYTES_PER_KB)
  return Math.max(MIN_IMAGE_TARGET_SIZE_IN_KB, kilobytes)
}
