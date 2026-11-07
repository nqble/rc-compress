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

/// "Compress by N%": `--compress-by=30` makes each file 30% smaller (a target size), with the Quick
/// Compress presets' other settings. Must match `hooks.nsh`.
pub const COMPRESS_BY_ARG: &str = "--compress-by=";

/// The percentages the menu offers; must match `hooks.nsh` and `src/utils/compressBy.ts`
/// (`scripts/test-compress-by.mjs` checks all three)
pub const COMPRESS_BY_PERCENTS: [u8; 9] = [10, 20, 30, 40, 50, 60, 70, 80, 90];

/// "Fit to size": `--fit-to=8` compresses each file to fit within 8 MB; `--fit-to=custom` within the
/// size set in the app (Settings -> Right-click Menu). Must match `hooks.nsh`.
pub const FIT_TO_ARG: &str = "--fit-to=";

/// The sizes the menu offers, in MB; must match `hooks.nsh` and `src/utils/compressBy.ts`
pub const FIT_SIZES_IN_MB: [u16; 3] = [8, 10, 25];

/// "Remove metadata": copies each file without its metadata (location, camera, dates), not
/// compressed. Must match `hooks.nsh`.
pub const REMOVE_METADATA_ARG: &str = "--remove-metadata";

/// What a launch from the right-click menu asks for
#[derive(Debug, PartialEq, Eq)]
pub enum MenuAction {
    QuickCompress,
    /// Smaller by this percentage
    CompressBy(u8),
    /// Within this many MB, or (`None`) the size set in the app
    FitTo(Option<u16>),
    RemoveMetadata,
}

/// The right-click menu action a launch asks for, if any (its files are the other arguments)
pub fn action_from_launch_args(args: &[String]) -> Option<MenuAction> {
    for arg in args.iter().skip(1) {
        if arg == QUICK_COMPRESS_ARG {
            return Some(MenuAction::QuickCompress);
        }
        if arg == REMOVE_METADATA_ARG {
            return Some(MenuAction::RemoveMetadata);
        }
        if let Some(percent) = compress_by_percent(arg) {
            return Some(MenuAction::CompressBy(percent));
        }
        if let Some(size) = fit_size(arg) {
            return Some(MenuAction::FitTo(size));
        }
    }
    None
}

/// Whether this app was started by the right-click menu (not already running): its main window then
/// stays hidden, as the file is compressed in the background (see `take_background_launch`)
pub fn is_menu_launch() -> bool {
    let args: Vec<String> = std::env::args().collect();
    action_from_launch_args(&args).is_some()
}

/// The percentage of a `--compress-by=N` option, when N is one the menu offers
fn compress_by_percent(arg: &str) -> Option<u8> {
    let percent = arg.strip_prefix(COMPRESS_BY_ARG)?.parse::<u8>().ok()?;
    COMPRESS_BY_PERCENTS.contains(&percent).then_some(percent)
}

/// The size of a `--fit-to=` option: one the menu offers (`Some(Some(MB))`) or `custom` (`Some(None)`)
fn fit_size(arg: &str) -> Option<Option<u16>> {
    let value = arg.strip_prefix(FIT_TO_ARG)?;
    if value == "custom" {
        return Some(None);
    }
    let megabytes = value.parse::<u16>().ok()?;
    FIT_SIZES_IN_MB.contains(&megabytes).then_some(Some(megabytes))
}

/// Queues the files of a "Quick Compress", "Compress by N%" or "Fit to size" launch (`menu`). Until the app's windows have told the backend
/// which media they can compress (on a launch straight from the menu, before they load), every
/// file is queued, and the queue skips what it cannot open.
#[cfg(windows)]
pub fn quick_compress(
    app_handle: &tauri::AppHandle,
    paths: Vec<String>,
    menu: crate::tauri_commands::drop_queue::MenuChoice,
) {
    use crate::tauri_commands::tray::{has_supported_drop_extensions, is_supported_drop};

    log::info!("[explorer-menu] Quick Compress of {} files", paths.len());
    let app = app_handle.clone();
    let is_supported =
        move |path: &str| !has_supported_drop_extensions(&app) || is_supported_drop(&app, path);
    crate::tauri_commands::drop_queue::add_to_quick_compress_queue(
        app_handle,
        paths,
        is_supported,
        menu,
    );
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
    fn compress_by_reads_the_offered_percentages_only() {
        let launch = args(&["rc-compress.exe", "--compress-by=30", "C:\\a.mp4"]);
        assert_eq!(action_from_launch_args(&launch), Some(MenuAction::CompressBy(30)));
        for unknown in ["--compress-by=35", "--compress-by=0", "--compress-by=100", "--compress-by=x"] {
            let launch = args(&["rc-compress.exe", unknown, "C:\\a.mp4"]);
            assert_eq!(action_from_launch_args(&launch), None, "{unknown}");
        }
    }

    #[test]
    fn fit_to_reads_the_offered_sizes_and_custom_only() {
        let launch = args(&["rc-compress.exe", "--fit-to=8", "C:\\a.mp4"]);
        assert_eq!(action_from_launch_args(&launch), Some(MenuAction::FitTo(Some(8))));
        let launch = args(&["rc-compress.exe", "--fit-to=custom", "C:\\a.mp4"]);
        assert_eq!(action_from_launch_args(&launch), Some(MenuAction::FitTo(None)));
        for unknown in ["--fit-to=9", "--fit-to=", "--fit-to=Custom"] {
            let launch = args(&["rc-compress.exe", unknown, "C:\\a.mp4"]);
            assert_eq!(action_from_launch_args(&launch), None, "{unknown}");
        }
    }

    #[test]
    fn remove_metadata_is_read() {
        let launch = args(&["rc-compress.exe", REMOVE_METADATA_ARG, "C:\\a.jpg"]);
        assert_eq!(action_from_launch_args(&launch), Some(MenuAction::RemoveMetadata));
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
