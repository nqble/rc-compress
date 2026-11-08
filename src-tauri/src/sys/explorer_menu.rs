//! The file manager's right-click menu (Windows Explorer for now): its entries launch the app with
//! an action option and the chosen file, e.g. `rc-compress.exe --quick-compress "C:\video.mp4"`.
//! The installer adds the entries (`src-tauri/windows/hooks.nsh`) and its uninstaller
//! removes them. Explorer starts the app once per chosen file; when the app is already running,
//! each launch reaches it as a second instance (see `on_second_instance` in `main.rs`).
//!
//! "Open in rc-compress…" is a plain launch with the file ("Open with"), so only the other actions
//! are handled here.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::compressible::is_compressible;
use crate::core::domain::CollectedFile;

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

/// Sent to the tray window when none of the files a menu launch chose can be compressed (see `take_skipped_note`)
pub const NOTHING_TO_COMPRESS_EVENT: &str = "nothing-to-compress";

/// How many files the last menu launches chose that could not be compressed, until the tray window shows its note
/// (it may load after the launch, so it asks with `take_skipped_note` as well as listening)
static SKIPPED_FOR_NOTE: AtomicUsize = AtomicUsize::new(0);

/// When a menu launch last queued a file. Explorer starts one launch per selected file, so a `.txt` launch can't see
/// the `.mp4` chosen with it: the note waits `NOTE_GRACE` and is shown only if no launch near it queued anything.
static LAST_QUEUED: Mutex<Option<Instant>> = Mutex::new(None);

/// Explorer's launches for one selection arrive within a fraction of a second (Master measured 0.2 s for 10)
const NOTE_GRACE: Duration = Duration::from_millis(1500);

/// The files a launch chose (folders already opened into their files) that can be compressed, and how many can't
fn split_compressible(files: Vec<CollectedFile>) -> (Vec<CollectedFile>, usize) {
    let (compressible, skipped): (Vec<_>, Vec<_>) =
        files.into_iter().partition(|file| is_compressible(&file.path));
    (compressible, skipped.len())
}

/// Whether the note for files skipped at `skipped_at` is due, checked `grace` later: not when a launch of the same
/// selection queued a file (at most `grace` before it, or any time since)
fn is_note_due(last_queued: Option<Instant>, skipped_at: Instant, grace: Duration) -> bool {
    match last_queued {
        None => true,
        Some(queued_at) => queued_at + grace < skipped_at,
    }
}

/// Queues the files of a "Quick Compress", "Compress by N%", "Fit to size" or "Remove metadata" launch (`menu`).
/// A chosen folder gives the media inside it, subfolders included (each copy is saved next to its original). Files
/// rc-compress cannot compress (a `.txt`, a `.zip`) are skipped; when that is all of them, the tray shows a note.
#[cfg(windows)]
pub fn quick_compress(
    app_handle: &tauri::AppHandle,
    paths: Vec<String>,
    menu: crate::tauri_commands::drop_queue::MenuChoice,
) {
    log::info!("[explorer-menu] Quick Compress of {} files or folders", paths.len());
    // Also allows the app to read them (asset scopes), as for dropped files & folders
    let files = match crate::tauri_commands::fs::allow_asset_scopes(app_handle, paths) {
        Ok(files) => files,
        Err(err) => {
            log::error!("[explorer-menu] Could not read the chosen files: {}", err);
            return;
        }
    };
    let (compressible, skipped) = split_compressible(files);
    if compressible.is_empty() {
        // An empty folder has nothing to say a note about
        if skipped > 0 {
            note_skipped_files(app_handle, skipped);
        }
        return;
    }
    if let Ok(mut last_queued) = LAST_QUEUED.lock() {
        *last_queued = Some(Instant::now());
    }
    crate::tauri_commands::drop_queue::add_files_to_quick_compress_queue(
        app_handle,
        compressible,
        menu,
    );
}

/// Counts the skipped files and, after `NOTE_GRACE`, has the tray show its note unless the selection queued a file
#[cfg(windows)]
fn note_skipped_files(app_handle: &tauri::AppHandle, count: usize) {
    log::info!("[explorer-menu] None of the {count} files can be compressed");
    SKIPPED_FOR_NOTE.fetch_add(count, Ordering::SeqCst);
    let skipped_at = Instant::now();
    let app_handle = app_handle.clone();
    std::thread::spawn(move || {
        std::thread::sleep(NOTE_GRACE);
        let last_queued = LAST_QUEUED.lock().ok().and_then(|last_queued| *last_queued);
        if !is_note_due(last_queued, skipped_at, NOTE_GRACE) {
            SKIPPED_FOR_NOTE.store(0, Ordering::SeqCst);
            return;
        }
        show_skipped_note(&app_handle);
    });
}

#[cfg(windows)]
fn show_skipped_note(app_handle: &tauri::AppHandle) {
    use tauri::Emitter;

    crate::tray_popover::ensure_tray_window(app_handle);
    let _ = app_handle.emit_to(
        crate::tray_popover::TRAY_WINDOW_LABEL,
        NOTHING_TO_COMPRESS_EVENT,
        (),
    );
}

/// The files the tray window has a note to show for (0 for none), cleared as it is taken
#[tauri::command]
pub fn take_skipped_note() -> usize {
    SKIPPED_FOR_NOTE.swap(0, Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|arg| arg.to_string()).collect()
    }

    fn file(path: &str) -> CollectedFile {
        CollectedFile { path: path.to_string(), relative_dir: String::new() }
    }

    #[test]
    fn only_media_and_pdfs_are_queued_and_the_rest_counted() {
        let files = vec![file("Trip/clip.mp4"), file("Trip/notes.txt"), file("Trip/Day1/a.jpg"), file("b.zip")];
        let (compressible, skipped) = split_compressible(files);
        let paths: Vec<_> = compressible.iter().map(|file| file.path.as_str()).collect();
        assert_eq!(paths, ["Trip/clip.mp4", "Trip/Day1/a.jpg"]);
        assert_eq!(skipped, 2);
    }

    #[test]
    fn nothing_compressible_in_an_empty_choice() {
        let (compressible, skipped) = split_compressible(Vec::new());
        assert!(compressible.is_empty());
        assert_eq!(skipped, 0);
    }

    #[test]
    fn no_note_when_the_same_selection_queued_a_file() {
        let grace = Duration::from_millis(1500);
        let skipped_at = Instant::now();
        assert!(is_note_due(None, skipped_at, grace));
        // A video chosen with the .txt, queued just before or just after it
        assert!(!is_note_due(Some(skipped_at - Duration::from_millis(300)), skipped_at, grace));
        assert!(!is_note_due(Some(skipped_at + Duration::from_millis(300)), skipped_at, grace));
        // A file queued long before, by another selection
        assert!(is_note_due(Some(skipped_at - Duration::from_secs(60)), skipped_at, grace));
    }

    #[test]
    fn the_note_is_taken_once() {
        SKIPPED_FOR_NOTE.store(2, Ordering::SeqCst);
        assert_eq!(take_skipped_note(), 2);
        assert_eq!(take_skipped_note(), 0);
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
