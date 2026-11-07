// Tests for "Compress by N%" (src/utils/compressBy.ts): the target sizes that make a file N% smaller, and that the
// menu, the app and these sizes agree on the percentages. Run with: pnpm test:compress-by
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'
import ts from 'typescript'

const read = (path) =>
  readFileSync(new URL(`../${path}`, import.meta.url), 'utf8')
const { outputText } = ts.transpileModule(read('src/utils/compressBy.ts'), {
  compilerOptions: { module: ts.ModuleKind.ESNext },
})
const {
  COMPRESS_BY_PERCENTS,
  FIT_SIZES_IN_MB,
  imageFitSizeInKB,
  imageTargetSizeInKB,
  videoTargetSizeInMB,
} = await import(
  `data:text/javascript;base64,${Buffer.from(outputText).toString('base64')}`
)

test('a video 30% smaller targets 70% of its size, in MB', () => {
  assert.equal(videoTargetSizeInMB(100_000_000, 30), 70)
  assert.equal(videoTargetSizeInMB(7_690_000, 50), 3.84)
})

test('an image 50% smaller targets half its size, in KB', () => {
  assert.equal(imageTargetSizeInKB(2_000_000, 50), 1000)
})

test('tiny files never go below the smallest target size', () => {
  assert.equal(videoTargetSizeInMB(500_000, 90), 1)
  assert.equal(imageTargetSizeInKB(20_000, 90), 10)
  const targetSize = read('src/utils/targetSize.ts')
  const imageTargetSize = read('src/utils/imageTargetSize.ts')
  assert.match(targetSize, /MIN_TARGET_SIZE_IN_MB = 1\b/)
  assert.match(imageTargetSize, /MIN_IMAGE_TARGET_SIZE_IN_KB = 10\b/)
})

test('the menu, the app and the sizes offer the same percentages', () => {
  const menu = read('src-tauri/windows/hooks.nsh')
  const menuPercents = [...menu.matchAll(/--compress-by=(\d+)/g)].map((m) =>
    Number(m[1]),
  )
  assert.deepEqual(menuPercents, COMPRESS_BY_PERCENTS)
  const rust = read('src-tauri/src/sys/explorer_menu.rs')
  assert.match(
    rust,
    /COMPRESS_BY_PERCENTS: \[u8; 9\] = \[10, 20, 30, 40, 50, 60, 70, 80, 90\]/,
  )
})

test('fit to size offers the same sizes in the menu, the app and here', () => {
  const menu = read('src-tauri/windows/hooks.nsh')
  const sizes = [...menu.matchAll(/--fit-to=(\d+)/g)].map((m) => Number(m[1]))
  assert.deepEqual(sizes, FIT_SIZES_IN_MB)
  assert.match(menu, /--fit-to=custom/)
  const rust = read('src-tauri/src/sys/explorer_menu.rs')
  assert.match(rust, /FIT_SIZES_IN_MB: \[u16; 3\] = \[8, 10, 25\]/)
})

test('an image fits within a size given in MB, counted in KB', () => {
  assert.equal(imageFitSizeInKB(8), 8000)
  assert.equal(imageFitSizeInKB(0.001), 10)
})
