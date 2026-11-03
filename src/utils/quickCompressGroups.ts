// The user's own Quick Compress groups (Settings -> Right-click Menu): file types with the preset
// "Quick Compress" uses for them, checked before the built-in video, image and audio presets.
// Kept free of imports so `scripts/test-quick-compress-groups.mjs` can run it on its own.

/** The kinds of media a preset is made for (`PresetMediaType`) */
type GroupMediaType = 'video' | 'image' | 'audio'

export type QuickCompressGroup = {
  id: string
  /** The user's name for it, e.g. "Screen recordings" */
  name: string
  /** A group holds one kind of media, as its preset is made for one kind */
  mediaType: GroupMediaType
  /** Lower case, without the dot, e.g. `mov` */
  extensions: string[]
  /** A preset choice, as `PresetChoiceSelect` gives it (Default unless chosen) */
  presetId: string
}

const MEDIA_TYPES: GroupMediaType[] = ['video', 'image', 'audio']

/** A file's extension, lower case and without the dot (`""` when it has none) */
export function extensionOf(path: string) {
  const name = path.split(/[\\/]/).pop() ?? ''
  const dot = name.lastIndexOf('.')
  if (dot <= 0) {
    return ''
  }
  return name.slice(dot + 1).toLowerCase()
}

/**
 * The extensions typed in a group's field ("mov, .MKV mp4"), lower case, without dots, each once,
 * keeping only those of the group's kind of media (`allowed`)
 */
export function parseExtensions(text: string, allowed: readonly string[]) {
  const words = text.split(/[\s,;]+/)
  const extensions = words.map((word) => word.replace(/^\./, '').toLowerCase())
  const known = extensions.filter((extension) => allowed.includes(extension))
  return Array.from(new Set(known))
}

/**
 * The first of the user's groups, in their order, that holds this file's type, for the kind of
 * media the file is compressed as; `undefined` falls back to the built-in preset of that kind
 */
export function findQuickCompressGroup(
  groups: readonly QuickCompressGroup[],
  path: string,
  mediaType: GroupMediaType,
) {
  const extension = extensionOf(path)
  return groups.find(
    (group) =>
      group.mediaType === mediaType && group.extensions.includes(extension),
  )
}

/** The stored groups, dropping any that are not valid */
export function readQuickCompressGroups(stored: unknown): QuickCompressGroup[] {
  if (!Array.isArray(stored)) {
    return []
  }
  return stored.filter(
    (group): group is QuickCompressGroup =>
      typeof group?.id === 'string' &&
      typeof group?.name === 'string' &&
      MEDIA_TYPES.includes(group?.mediaType) &&
      Array.isArray(group?.extensions) &&
      group.extensions.every(
        (extension: unknown) => typeof extension === 'string',
      ) &&
      typeof group?.presetId === 'string',
  )
}
