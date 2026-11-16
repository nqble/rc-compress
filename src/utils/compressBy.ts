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

/**
 * "Fit to size" in the right-click menu: the sizes it offers, in MB; must match `explorer_menu.rs` and `hooks.nsh`
 * (`scripts/test-compress-by.mjs` checks all three). "Your size" is `settingsStore.quickCompressFitSizeInMB`.
 */
export const FIT_SIZES_IN_MB = [8, 10, 25]

/** The default of "Your size", in MB */
export const DEFAULT_CUSTOM_FIT_SIZE_IN_MB = 50

/**
 * Whether a file of `sizeInBytes` already fits within `megabytes`. Its target size then asks for nothing, and searching
 * for it takes long with nothing to show (half a minute for a 5.5 MB video and 10 MB), so the preset compresses it as
 * Quick Compress does. An unknown size doesn't fit.
 */
export function alreadyFits(
  sizeInBytes: number | null | undefined,
  megabytes: number,
) {
  return (
    sizeInBytes != null &&
    sizeInBytes > 0 &&
    sizeInBytes <= megabytes * BYTES_PER_MB
  )
}

/** An image's target to fit within `megabytes`, in KB */
export function imageFitSizeInKB(megabytes: number) {
  return Math.max(
    MIN_IMAGE_TARGET_SIZE_IN_KB,
    Math.floor((megabytes * BYTES_PER_MB) / BYTES_PER_KB),
  )
}
