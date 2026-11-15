// Which systems have the switch "Show in the file manager's menu" (Settings -> Right-click Menu)
import { getPlatform } from '@/utils/fs'

const { isMacOS, isLinux } = getPlatform()

/** macOS & Linux: the app adds the entries itself; on Windows the installer does */
export const hasFileManagerMenuSwitch = isMacOS || isLinux

/** Picks the switch's wording (`title_macos`, `title_linux`, …) */
export const fileManagerMenuContext = isMacOS ? 'macos' : 'linux'
