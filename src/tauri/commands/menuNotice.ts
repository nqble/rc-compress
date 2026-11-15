import { core } from '@tauri-apps/api'

/** Must match `MENU_NOTICE_WINDOW_LABEL` in `menu_notice.rs` */
export const MENU_NOTICE_WINDOW_LABEL = 'menu-notice'

/** Closes the notice; an app a menu launch started (no window open) then quits */
export function closeMenuNotice() {
  return core.invoke<void>('close_menu_notice')
}

/** Shows the main window, with the queue, & closes the notice */
export function openMainWindowFromNotice() {
  return core.invoke<void>('open_main_window_from_notice')
}

/** How many files a menu launch chose that can't be compressed (0 for none), cleared as it is taken */
export function takeSkippedNote() {
  return core.invoke<number>('take_skipped_note')
}
