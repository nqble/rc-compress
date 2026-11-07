// Tests for the PDF levels and the right-click menu's PDF sizes (src/utils/pdfLevels.ts), and that the levels match
// the Rust side (pdf.rs). Run with: pnpm test:pdf-levels
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'
import ts from 'typescript'

const read = (path) =>
  readFileSync(new URL(`../${path}`, import.meta.url), 'utf8')
const { outputText } = ts.transpileModule(read('src/utils/pdfLevels.ts'), {
  compilerOptions: { module: ts.ModuleKind.ESNext },
})
const levels = await import(
  `data:text/javascript;base64,${Buffer.from(outputText).toString('base64')}`
)

const quick = { targetPercent: null, fitToMb: null, fitToCustom: false }

test('Quick Compress has no target size', () => {
  assert.equal(levels.pdfTargetBytes(quick, 5_000_000, 50), null)
})

test('Compress by N% targets what is left of the original', () => {
  const choice = { ...quick, targetPercent: 40 }
  assert.equal(levels.pdfTargetBytes(choice, 5_000_000, 50), 3_000_000)
})

test('Fit to size targets the size, or Your size', () => {
  assert.equal(
    levels.pdfTargetBytes({ ...quick, fitToMb: 8 }, 50_000_000, 50),
    8_000_000,
  )
  assert.equal(
    levels.pdfTargetBytes({ ...quick, fitToCustom: true }, 90_000_000, 50),
    50_000_000,
  )
})

test('the levels are tried from the largest output down', () => {
  assert.deepEqual(levels.PDF_LEVELS_LARGEST_FIRST, ['high', 'medium', 'small'])
  assert.equal(levels.fitsTarget(3_000_000, 3_000_000), true)
  assert.equal(levels.fitsTarget(3_000_001, 3_000_000), false)
})

test('the levels match the Rust side', () => {
  const rust = read('src-tauri/src/tauri_commands/pdf.rs')
  for (const level of levels.PDF_LEVELS) {
    const name = level[0].toUpperCase() + level.slice(1)
    assert.match(rust, new RegExp(`PdfLevel::${name} =>`), name)
  }
  assert.equal(levels.isPdfLevel('medium'), true)
  assert.equal(levels.isPdfLevel('tiny'), false)
})
