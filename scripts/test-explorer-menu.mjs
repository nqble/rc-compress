// Checks that the installer's right-click menu (src-tauri/windows/hooks.nsh) launches the
// app with the options the app reads (src-tauri/src/sys/explorer_menu.rs), and that the installer
// is set to use it. Run with: pnpm test:explorer-menu
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import { test } from 'node:test'

const read = (path) =>
  readFileSync(new URL(`../${path}`, import.meta.url), 'utf8')

const menu = read('src-tauri/windows/hooks.nsh')
const rust = read('src-tauri/src/sys/explorer_menu.rs')

test('every option the menu passes is one the app reads', () => {
  const options = [...menu.matchAll(/\.exe" (--[a-z-]+)/g)].map(
    (match) => match[1],
  )
  assert.ok(options.length > 0, 'the menu passes no option')
  for (const option of options) {
    assert.ok(
      rust.includes(`"${option}"`),
      `${option} is not read by explorer_menu.rs`,
    )
  }
})

test('the uninstaller removes the key the installer writes', () => {
  assert.match(
    menu,
    /!macro RC_MENU_INSTALL[\s\S]*WriteRegStr HKCU "\$\{RC_MENU_KEY\}"/,
  )
  assert.match(
    menu,
    /!macro RC_MENU_UNINSTALL[\s\S]*DeleteRegKey HKCU "\$\{RC_MENU_KEY\}"/,
  )
})

test('the install and uninstall hooks run the menu steps', () => {
  assert.match(
    menu,
    /!macro NSIS_HOOK_POSTINSTALL\n\s*!insertmacro RC_MENU_INSTALL/,
  )
  assert.match(
    menu,
    /!macro NSIS_HOOK_PREUNINSTALL\n\s*!insertmacro RC_MENU_UNINSTALL/,
  )
})

test('the installer is set to use the menu hooks', () => {
  const config = JSON.parse(read('src-tauri/tauri.conf.json'))
  assert.equal(config.bundle.windows.nsis.installerHooks, 'windows/hooks.nsh')
})
