//! The brief notice a right-click menu launch shows on Linux, which has no tray icon to show
//! progress on: a small window in the bottom-right corner of the screen that says what is being
//! compressed, then that it's done (or that nothing could be), and closes itself a few seconds later
//! (`src/routes/webviews/menu-notice/`). The main window compresses the queue meanwhile, hidden.
//! When it's already open, it shows the queue itself and no notice is needed.
//!
//! Windows & macOS show the same in the tray's panel instead (see `explorer_menu.rs`).
use tauri::{AppHandle, Manager, PhysicalPosition, WebviewUrl, WebviewWindowBuilder};

/// Must match `MENU_NOTICE_WINDOW_LABEL` in `src/routes/__root.tsx` & `capabilities/menu-notice.json`
pub const MENU_NOTICE_WINDOW_LABEL: &str = "menu-notice";
const MENU_NOTICE_URL: &str = "webviews/menu-notice";

/// Its size in logical pixels: room for three lines of text (the longest message) & two buttons
const NOTICE_WIDTH: f64 = 360.0;
const NOTICE_HEIGHT: f64 = 136.0;
/// Its distance from the screen's edges, in logical pixels (clear of a bottom panel)
const SCREEN_MARGIN: f64 = 48.0;

const MAIN_WINDOW_LABEL: &str = "main";

/// Shows the notice, unless it or the main window is already showing (both follow the queue)
pub fn show(app_handle: &AppHandle) {
    if is_main_window_visible(app_handle)
        || app_handle
            .get_webview_window(MENU_NOTICE_WINDOW_LABEL)
            .is_some()
    {
        return;
    }
    if let Err(err) = create(app_handle) {
        log::error!("[menu-notice] Could not show the notice: {}", err);
    }
}

fn is_main_window_visible(app_handle: &AppHandle) -> bool {
    app_handle
        .get_webview_window(MAIN_WINDOW_LABEL)
        .map_or(false, |window| window.is_visible().unwrap_or(false))
}

fn create(app_handle: &AppHandle) -> Result<(), String> {
    let window = WebviewWindowBuilder::new(
        app_handle,
        MENU_NOTICE_WINDOW_LABEL,
        WebviewUrl::App(MENU_NOTICE_URL.into()),
    )
    .title("rc-compress")
    .decorations(false)
    .resizable(false)
    .maximizable(false)
    .minimizable(false)
    .skip_taskbar(true)
    .always_on_top(true)
    // Not to take the keyboard from the file manager the person is using
    .focused(false)
    .inner_size(NOTICE_WIDTH, NOTICE_HEIGHT)
    .build()
    .map_err(|err| err.to_string())?;
    let Some(monitor) = window.primary_monitor().ok().flatten() else {
        return Ok(());
    };
    let size = monitor.size();
    let (x, y) = corner_position(size.width, size.height, monitor.scale_factor());
    let _ = window.set_position(PhysicalPosition::new(x, y));
    Ok(())
}

/// Where the notice's top-left corner goes, in physical pixels, for a screen of that size
fn corner_position(screen_width: u32, screen_height: u32, scale_factor: f64) -> (i32, i32) {
    let right = f64::from(screen_width) - (NOTICE_WIDTH + SCREEN_MARGIN) * scale_factor;
    let bottom = f64::from(screen_height) - (NOTICE_HEIGHT + SCREEN_MARGIN) * scale_factor;
    (
        right.max(0.0).round() as i32,
        bottom.max(0.0).round() as i32,
    )
}

/// The notice is done (or the person closed it). An app that a menu launch started has its main
/// window hidden & nothing else to show, so it quits, as it would once its window is closed.
#[tauri::command]
pub fn close_menu_notice(app_handle: AppHandle) {
    if let Some(window) = app_handle.get_webview_window(MENU_NOTICE_WINDOW_LABEL) {
        let _ = window.destroy();
    }
    if !is_main_window_visible(&app_handle) {
        log::info!("[menu-notice] Done, and no window is open: quitting");
        app_handle.exit(0);
    }
}

/// "Open rc-compress" on the notice: the main window, with the queue, in its place
#[tauri::command]
pub fn open_main_window_from_notice(app_handle: AppHandle) {
    if let Some(window) = app_handle.get_webview_window(MAIN_WINDOW_LABEL) {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
    }
    if let Some(notice) = app_handle.get_webview_window(MENU_NOTICE_WINDOW_LABEL) {
        let _ = notice.destroy();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sits_in_the_bottom_right_corner() {
        assert_eq!(corner_position(1920, 1080, 1.0), (1512, 896));
        // A HiDPI screen: the same place, in twice the pixels
        assert_eq!(corner_position(3840, 2160, 2.0), (3024, 1792));
    }

    #[test]
    fn a_tiny_screen_keeps_it_on_screen() {
        assert_eq!(corner_position(200, 100, 1.0), (0, 0));
    }
}
