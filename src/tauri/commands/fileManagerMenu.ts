import { core } from '@tauri-apps/api'

/**
 * Adds or removes rc-compress's entries in the file manager's right-click menu: Finder's Quick
 * Actions on macOS, Dolphin, Files & Nemo on Linux (see `file_manager_menu.rs`)
 */
export function setFileManagerMenuEnabled(enabled: boolean) {
  return core.invoke<void>('set_file_manager_menu_enabled', { enabled })
}
