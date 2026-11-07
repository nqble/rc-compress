// The name a compressed copy is saved under. With a prefix (Settings -> Output, or a preset's own), it is the prefix
// and the original name, as in compressO ("small-video.mp4"). With no prefix, " (compressed)" goes before the
// extension ("video (compressed).mp4"), so the copy never takes the original's name: rc-compress's default, as its
// copies are saved next to the originals. With "Replace original files" on, there is no suffix: that option replaces
// an original only by a copy of the same name. Kept free of imports so `scripts/test-output-file-name.mjs` can run it.

/** Added before the extension when there is no prefix */
export const COMPRESSED_SUFFIX = ' (compressed)'

/** The name the copy of `name` is saved under, with `prefix` (see the header) */
export function outputFileName(
  prefix: string,
  name: string,
  replacesOriginal = false,
) {
  if (prefix || replacesOriginal) {
    return `${prefix}${name}`
  }
  const dot = name.lastIndexOf('.')
  if (dot <= 0) {
    return `${name}${COMPRESSED_SUFFIX}`
  }
  return `${name.slice(0, dot)}${COMPRESSED_SUFFIX}${name.slice(dot)}`
}
