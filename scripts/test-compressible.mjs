// Checks that the file types the right-click menu queues (src-tauri/src/sys/compressible.rs) are the app's own
// (`inputExtensions` in src/types/compression.ts) plus PDFs, so the menu never skips media the app can compress, or
// queues files it can't. Run with: pnpm test:compressible
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'
import ts from 'typescript'

const read = (path) =>
  readFileSync(new URL(`../${path}`, import.meta.url), 'utf8')

// The module's only other import is the translations, which the lists don't use
const source = read('src/types/compression.ts').replace(
  /^import \{ t \} from '@\/i18n'$/m,
  'const t = (key) => key',
)
const { outputText } = ts.transpileModule(source, {
  compilerOptions: { module: ts.ModuleKind.ESNext },
})
const { inputExtensions } = await import(
  `data:text/javascript;base64,${Buffer.from(outputText).toString('base64')}`
)

function rustExtensions() {
  const rust = read('src-tauri/src/sys/compressible.rs')
  const list = /COMPRESSIBLE_EXTENSIONS: \[&str; \d+\] = \[([\s\S]*?)\];/.exec(
    rust,
  )
  assert.ok(list, 'COMPRESSIBLE_EXTENSIONS not found in compressible.rs')
  return [...list[1].matchAll(/"([a-z0-9]+)"/g)].map((match) => match[1])
}

test('the menu queues exactly the media the app opens, and PDFs', () => {
  const app = new Set([
    ...inputExtensions.video,
    ...inputExtensions.image,
    ...inputExtensions.audio,
    'pdf',
  ])
  assert.deepEqual(new Set(rustExtensions()), app)
})

test('each type is listed once', () => {
  const extensions = rustExtensions()
  assert.equal(new Set(extensions).size, extensions.length)
})
