// Checks what the brief notice of a right-click menu launch says on Linux (src/utils/menuNotice.ts):
// how far the queue is, then how many were compressed or failed.
// Run with: pnpm test:menu-notice
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'
import ts from 'typescript'

const source = readFileSync(
  new URL('../src/utils/menuNotice.ts', import.meta.url),
  'utf8',
)
const { outputText } = ts.transpileModule(source, {
  compilerOptions: { module: ts.ModuleKind.ESNext },
})
const { summarize } = await import(
  `data:text/javascript;base64,${Buffer.from(outputText).toString('base64')}`
)

const item = (id, status, progress = 0) => ({ id, status, progress })

test('compressing: counts what is left and the progress of all of it', () => {
  const items = [item('a', 'done', 100), item('b', 'compressing', 50)]
  assert.deepEqual(summarize(items), {
    state: 'compressing',
    count: 1,
    progress: 75,
  })
})

test('done: how many were compressed and how many failed', () => {
  const items = [item('a', 'done'), item('b', 'failed'), item('c', 'done')]
  assert.deepEqual(summarize(items), {
    state: 'done',
    done: 2,
    failed: 1,
  })
})

test('media finished before the notice loaded still counts (a small file is quick)', () => {
  const items = [item('a', 'done'), item('b', 'done')]
  assert.deepEqual(summarize(items), { state: 'done', done: 2, failed: 0 })
})

test('still waiting for the queue to arrive', () => {
  assert.deepEqual(summarize([]), { state: 'waiting' })
})
