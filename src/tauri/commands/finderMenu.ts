import { core } from '@tauri-apps/api'

/** Adds or removes rc-compress's Quick Actions in Finder (macOS, see `finder_menu.rs`) */
export function setFinderMenuEnabled(enabled: boolean) {
  return core.invoke<void>('set_finder_menu_enabled', { enabled })
}
