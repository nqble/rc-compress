#[cfg(any(target_os = "macos", windows))]
pub mod clipboard_watch;
// The right-click menu's actions are parsed on every system (tested in CI), handled on Windows
#[cfg_attr(not(windows), allow(dead_code))]
pub mod explorer_menu;
pub mod fs;
pub mod history;
pub mod keep_awake;
#[cfg(target_os = "linux")]
pub mod linux_clipboard;
pub mod logging;
pub mod native_text;
pub mod output_registry;
pub mod quit;
pub mod watch_folders;
