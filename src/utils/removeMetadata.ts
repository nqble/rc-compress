// "Remove metadata" in the right-click menu (see `explorer_menu.rs`): the settings that copy a file without its
// metadata (location, camera, dates) and without compressing it, using compressO's own switches: a video or audio
// file is copied as is (`shouldDisableCompression`, its streams untouched), an image is saved losslessly. The preset's
// other edits (size, frame rate, target size) are turned off. Kept free of imports so
// `scripts/test-remove-metadata.mjs` can run it.

type MetadataFields = {
  shouldStripMetadata?: boolean
  metadataConfig?: unknown
  shouldUseTargetSize?: boolean
  shouldEnableCustomDimensions?: boolean
  shouldEnableCustomFPS?: boolean
  shouldDisableCompression?: boolean
  isLossless?: boolean
}

/** `config` changed to only remove the metadata of an image (`isImage`) or a video or audio file */
export function metadataOnlyConfig<Config extends MetadataFields>(
  config: Config,
  isImage: boolean,
): Config {
  const withoutMetadata = {
    ...config,
    shouldStripMetadata: true,
    metadataConfig: null,
    shouldUseTargetSize: false,
    shouldEnableCustomDimensions: false,
  }
  if (isImage) {
    return { ...withoutMetadata, isLossless: true }
  }
  return {
    ...withoutMetadata,
    shouldDisableCompression: true,
    shouldEnableCustomFPS: false,
  }
}
