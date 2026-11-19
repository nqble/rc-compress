// Build regeneration must not conflict with Biome; handwritten imports stay checked.
import assert from 'node:assert/strict'
import { spawnSync } from 'node:child_process'
import { test } from 'node:test'

const biome = 'node_modules/@biomejs/biome/bin/biome'
function run(script, args, input) {
  return spawnSync(process.execPath, [script, ...args], {
    encoding: 'utf8',
    input,
    maxBuffer: 10 * 1024 * 1024,
  })
}

test('a generated route tree passes Biome after a real build', () => {
  const build = run('node_modules/vite/bin/vite.js', ['build'])
  assert.equal(build.status, 0, build.stdout + build.stderr)
  const check = run(biome, ['ci', 'src/routeTree.gen.ts'])
  assert.equal(check.status, 0, check.stdout + check.stderr)
})

test('handwritten route imports still require Biome ordering', () => {
  const source =
    "import { z } from './z'\nimport { a } from './a'\n\nexport { z, a }\n"
  const check = run(
    biome,
    ['check', '--write', '--stdin-file-path=src/routes/route-order-control.ts'],
    source,
  )
  assert.equal(check.status, 0, check.stdout + check.stderr)
  assert.ok(
    check.stdout.indexOf('import { a }') < check.stdout.indexOf('import { z }'),
  )
})
