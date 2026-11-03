// Tests for the user's Quick Compress groups (src/utils/quickCompressGroups.ts): which group a file
// falls under, how typed extensions are read, and which stored groups are kept.
// Run with: pnpm test:quick-compress-groups
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'
import ts from 'typescript'

const source = readFileSync(
  new URL('../src/utils/quickCompressGroups.ts', import.meta.url),
  'utf8',
)
const { outputText } = ts.transpileModule(source, {
  compilerOptions: { module: ts.ModuleKind.ESNext },
})
const groupsModule = await import(
  `data:text/javascript;base64,${Buffer.from(outputText).toString('base64')}`
)
const {
  extensionOf,
  findQuickCompressGroup,
  parseExtensions,
  readQuickCompressGroups,
} = groupsModule

const screenRecordings = {
  id: 'a',
  name: 'Screen recordings',
  mediaType: 'video',
  extensions: ['mov', 'mkv'],
  presetId: 'small',
}
const allVideos = {
  id: 'b',
  name: 'All videos',
  mediaType: 'video',
  extensions: ['mov', 'mp4'],
  presetId: 'other',
}
const podcasts = {
  id: 'c',
  name: 'MP3',
  mediaType: 'audio',
  extensions: ['mp3'],
  presetId: 'podcast',
}

test('the first group holding the file type wins, in the user order', () => {
  const groups = [screenRecordings, allVideos, podcasts]
  assert.equal(
    findQuickCompressGroup(groups, 'C:\\clips\\a.MOV', 'video'),
    screenRecordings,
  )
  assert.equal(
    findQuickCompressGroup(groups, '/home/me/b.mp4', 'video'),
    allVideos,
  )
  assert.equal(
    findQuickCompressGroup(groups, 'C:\\talk.mp3', 'audio'),
    podcasts,
  )
})

test('no group, or a group of another kind of media, falls back to the built-in preset', () => {
  const groups = [screenRecordings, podcasts]
  assert.equal(
    findQuickCompressGroup(groups, 'C:\\photo.png', 'image'),
    undefined,
  )
  // An mp3 compressed as a video (it is not, but a group must match the kind too)
  assert.equal(
    findQuickCompressGroup(groups, 'C:\\talk.mp3', 'video'),
    undefined,
  )
  assert.equal(findQuickCompressGroup([], 'C:\\a.mov', 'video'), undefined)
})

test('a file type is read from the name, not the folders', () => {
  assert.equal(extensionOf('C:\\my.folder\\video'), '')
  assert.equal(extensionOf('/home/me/.hidden'), '')
  assert.equal(extensionOf('C:\\a.b.WebM'), 'webm')
})

test('typed extensions are cleaned, kept once, and limited to the group kind', () => {
  const videoTypes = ['mov', 'mkv', 'mp4']
  assert.deepEqual(parseExtensions('mov, .MKV  mp4;mov', videoTypes), [
    'mov',
    'mkv',
    'mp4',
  ])
  assert.deepEqual(parseExtensions('png, mov', videoTypes), ['mov'])
  assert.deepEqual(parseExtensions('', videoTypes), [])
})

test('stored groups that are not valid are dropped', () => {
  assert.deepEqual(readQuickCompressGroups('nope'), [])
  assert.deepEqual(
    readQuickCompressGroups([
      screenRecordings,
      { ...podcasts, mediaType: 'pdf' },
      { ...allVideos, extensions: 'mp4' },
      null,
    ]),
    [screenRecordings],
  )
})
