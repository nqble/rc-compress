#[cfg(any(target_os = "macos", windows))]
pub mod clipboard_watch;
// The right-click menu's actions are parsed on every system (tested in CI), handled on Windows and macOS
#[cfg_attr(target_os = "linux", allow(dead_code))]
pub mod explorer_menu;
pub mod cli_install;
pub mod cli_server;
pub mod cli_settings;
pub mod compressible;
pub mod early_launches;
// The Quick Actions' files are made & tested on every system, written on macOS
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub mod finder_menu;
pub mod fs;
pub mod history;
pub mod keep_awake;
// The launch wait's names and wait are tested on every system, used on Windows
#[cfg_attr(not(windows), allow(dead_code))]
pub mod launch_wait;
#[cfg(target_os = "linux")]
pub mod linux_clipboard;
pub mod logging;
pub mod native_text;
pub mod output_registry;
pub mod quit;
pub mod watch_folders;
