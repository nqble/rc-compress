// Tests for how many files of each kind the background queue compresses at once
// (src/utils/queueLanes.ts). Run with: pnpm test:queue-lanes
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'
import ts from 'typescript'

const source = readFileSync(
  new URL('../src/utils/queueLanes.ts', import.meta.url),
  'utf8',
)
const { outputText } = ts.transpileModule(source, {
  compilerOptions: { module: ts.ModuleKind.ESNext },
})
const { laneWorkers, MAX_IMAGE_WORKERS } = await import(
  `data:text/javascript;base64,${Buffer.from(outputText).toString('base64')}`
)

test('videos, audio and PDFs stay one at a time', () => {
  for (const lane of ['video', 'audio', 'pdf']) {
    assert.equal(laneWorkers(lane, 16), 1)
  }
})

test('images run one per four cores, at least one, at most the maximum', () => {
  assert.equal(laneWorkers('image', 16), 4)
  assert.equal(laneWorkers('image', 8), 2)
  assert.equal(laneWorkers('image', 2), 1)
  assert.equal(laneWorkers('image', 1), 1)
  assert.equal(laneWorkers('image', 64), MAX_IMAGE_WORKERS)
})
