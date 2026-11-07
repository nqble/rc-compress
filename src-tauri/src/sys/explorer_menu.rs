//! The file manager's right-click menu (Windows Explorer for now): its entries launch the app with
//! an action option and the chosen file, e.g. `rc-compress.exe --quick-compress "C:\video.mp4"`.
//! The installer adds the entries (`src-tauri/windows/hooks.nsh`) and its uninstaller
//! removes them. Explorer starts the app once per chosen file; when the app is already running,
//! each launch reaches it as a second instance (see `on_second_instance` in `main.rs`).
//!
//! "Open in rc-compress…" is a plain launch with the file ("Open with"), so only the other actions
//! are handled here.

/// Compress the files with the Quick Compress presets (Settings -> Right-click menu), in the
/// background queue, without opening the main window. Must match `hooks.nsh`.
pub const QUICK_COMPRESS_ARG: &str = "--quick-compress";

/// What a launch from the right-click menu asks for
#[derive(Debug, PartialEq, Eq)]
pub enum MenuAction {
    QuickCompress,
}

/// The right-click menu action a launch asks for, if any (its files are the other arguments)
pub fn action_from_launch_args(args: &[String]) -> Option<MenuAction> {
    let has_quick_compress = args.iter().skip(1).any(|arg| arg == QUICK_COMPRESS_ARG);
    if has_quick_compress {
        return Some(MenuAction::QuickCompress);
    }
    None
}

/// Queues the files of a "Quick Compress" launch. Until the app's windows have told the backend
/// which media they can compress (on a launch straight from the menu, before they load), every
/// file is queued, and the queue skips what it cannot open.
#[cfg(windows)]
pub fn quick_compress(app_handle: &tauri::AppHandle, paths: Vec<String>) {
    use crate::tauri_commands::tray::{has_supported_drop_extensions, is_supported_drop};

    log::info!("[explorer-menu] Quick Compress of {} files", paths.len());
    let app = app_handle.clone();
    crate::tauri_commands::drop_queue::add_to_quick_compress_queue(app_handle, paths, move |path| {
        !has_supported_drop_extensions(&app) || is_supported_drop(&app, path)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|arg| arg.to_string()).collect()
    }

    #[test]
    fn quick_compress_is_found_before_or_after_the_file() {
        for launch in [
            args(&["rc-compress.exe", QUICK_COMPRESS_ARG, "C:\\a.mp4"]),
            args(&["rc-compress.exe", "C:\\a.mp4", QUICK_COMPRESS_ARG]),
        ] {
            assert_eq!(
                action_from_launch_args(&launch),
                Some(MenuAction::QuickCompress)
            );
        }
    }

    #[test]
    fn a_plain_launch_or_open_with_has_no_action() {
        assert_eq!(action_from_launch_args(&args(&["rc-compress.exe"])), None);
        assert_eq!(
            action_from_launch_args(&args(&["rc-compress.exe", "C:\\a.mp4"])),
            None
        );
    }

    #[test]
    fn the_program_path_is_never_read_as_an_action() {
        assert_eq!(
            action_from_launch_args(&args(&[QUICK_COMPRESS_ARG, "C:\\a.mp4"])),
            None
        );
    }
}
