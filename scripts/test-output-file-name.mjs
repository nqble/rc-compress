// Tests for the name a compressed copy is saved under (src/utils/outputFileName.ts).
// Run with: pnpm test:output-file-name
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'
import ts from 'typescript'

const source = readFileSync(
  new URL('../src/utils/outputFileName.ts', import.meta.url),
  'utf8',
)
const { outputText } = ts.transpileModule(source, {
  compilerOptions: { module: ts.ModuleKind.ESNext },
})
const { outputFileName } = await import(
  `data:text/javascript;base64,${Buffer.from(outputText).toString('base64')}`
)

test('with no prefix, (compressed) goes before the extension', () => {
  assert.equal(outputFileName('', 'video.mp4'), 'video (compressed).mp4')
  assert.equal(outputFileName('', 'my.trip.MOV'), 'my.trip (compressed).MOV')
})

test('a name without an extension gets (compressed) at the end', () => {
  assert.equal(outputFileName('', 'README'), 'README (compressed)')
  assert.equal(outputFileName('', '.hidden'), '.hidden (compressed)')
})

test('with a prefix, the name is the prefix and the original name', () => {
  assert.equal(outputFileName('small-', 'video.mp4'), 'small-video.mp4')
})

test('with Replace original files on, the copy keeps the original name', () => {
  assert.equal(outputFileName('', 'video.mp4', true), 'video.mp4')
})
