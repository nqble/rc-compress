import { useEffect } from 'react'
import { subscribe } from 'valtio'

import { settingsStore } from '@/stores/settingsStore'
import { setFinderMenuEnabled } from '@/tauri/commands/finderMenu'
import { getPlatform } from '@/utils/fs'

/**
 * Adds or removes the Quick Actions in Finder (Settings -> Right-click Menu, macOS only): at each
 * start, so they follow the app when it is moved, & when the switch changes
 */
export function useFinderMenuSync() {
  useEffect(() => {
    if (!getPlatform().isMacOS) return
    let lastSent: boolean | null = null
    const apply = () => {
      const { finderMenuEnabled } = settingsStore
      if (finderMenuEnabled === lastSent) return
      lastSent = finderMenuEnabled
      setFinderMenuEnabled(finderMenuEnabled).catch(() => {})
    }
    apply()
    return subscribe(settingsStore, apply)
  }, [])
}
