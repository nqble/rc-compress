// Tests for "Remove metadata" (src/utils/removeMetadata.ts): which settings copy a file without its metadata.
// Run with: pnpm test:remove-metadata
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'
import ts from 'typescript'

const source = readFileSync(
  new URL('../src/utils/removeMetadata.ts', import.meta.url),
  'utf8',
)
const { outputText } = ts.transpileModule(source, {
  compilerOptions: { module: ts.ModuleKind.ESNext },
})
const { metadataOnlyConfig } = await import(
  `data:text/javascript;base64,${Buffer.from(outputText).toString('base64')}`
)

const presetVideo = {
  convertToExtension: 'mp4',
  shouldStripMetadata: false,
  metadataConfig: { title: 'Trip' },
  shouldUseTargetSize: true,
  shouldEnableCustomDimensions: true,
  shouldEnableCustomFPS: true,
  shouldDisableCompression: false,
}

test('a video is copied as is, without its metadata or the preset edits', () => {
  const config = metadataOnlyConfig(presetVideo, false)
  assert.equal(config.shouldDisableCompression, true)
  assert.equal(config.shouldStripMetadata, true)
  assert.equal(config.metadataConfig, null)
  assert.equal(config.shouldUseTargetSize, false)
  assert.equal(config.shouldEnableCustomDimensions, false)
  assert.equal(config.shouldEnableCustomFPS, false)
  assert.equal(config.convertToExtension, 'mp4')
})

test('an image is saved losslessly, without its metadata', () => {
  const config = metadataOnlyConfig(
    { convertToExtension: '-', isLossless: false, shouldStripMetadata: false },
    true,
  )
  assert.equal(config.isLossless, true)
  assert.equal(config.shouldStripMetadata, true)
  assert.equal(config.convertToExtension, '-')
})
