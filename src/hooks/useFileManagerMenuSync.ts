import { useEffect } from 'react'
import { subscribe } from 'valtio'

import { settingsStore } from '@/stores/settingsStore'
import { setFileManagerMenuEnabled } from '@/tauri/commands/fileManagerMenu'
import { hasFileManagerMenuSwitch } from '@/utils/fileManagerMenu'

/**
 * Adds or removes rc-compress's entries in the file manager's menu (Settings -> Right-click Menu,
 * macOS & Linux): at each start, so they follow the app when it is moved, & when the switch changes
 */
export function useFileManagerMenuSync() {
  useEffect(() => {
    if (!hasFileManagerMenuSwitch) return
    let lastSent: boolean | null = null
    const apply = () => {
      const { fileManagerMenuEnabled } = settingsStore
      if (fileManagerMenuEnabled === lastSent) return
      lastSent = fileManagerMenuEnabled
      setFileManagerMenuEnabled(fileManagerMenuEnabled).catch(() => {})
    }
    apply()
    return subscribe(settingsStore, apply)
  }, [])
}
