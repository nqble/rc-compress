//! Waits, at a launch, for an rc-compress that is still starting, so this launch's files reach it.
//!
//! When several files are opened at once (Explorer starts the app once per selected file), every launch but the
//! first must hand its files to the first and quit; `tauri-plugin-single-instance` does that through a hidden window
//! of the first instance. Its version here (2.3.7) looks for that window once: a launch that looks before the first
//! instance has made it starts a second, broken app, and its files are lost (seen on Windows, 2026-11-07: 1 of 5
//! files compressed when the app was closed). Version 2.5 waits for the window, but needs Tauri 2.12. This does the
//! same wait before the plugin looks, and can be deleted once the plugin is 2.5 or newer. Windows only; the names
//! and the wait are plain code, so their tests run everywhere.

use std::time::Duration;

#[cfg(windows)]
use windows_sys::Win32::Foundation::CloseHandle;
#[cfg(windows)]
use windows_sys::Win32::System::Threading::{OpenMutexW, SYNCHRONIZATION_SYNCHRONIZE};
#[cfg(windows)]
use windows_sys::Win32::UI::WindowsAndMessaging::FindWindowW;

/// The app's identifier (`tauri.conf.json`), which the plugin names its mutex & window after
const IDENTIFIER: &str = "com.rccompress.app";

/// As long as the plugin's own wait in 2.5 (40 tries, 25 ms apart)
const TRIES: u32 = 40;
const TRY_INTERVAL: Duration = Duration::from_millis(25);

/// The plugin's names for its mutex, window class & window (`platform_impl/windows.rs`)
fn names() -> (String, String, String) {
    (
        format!("{IDENTIFIER}-sim"),
        format!("{IDENTIFIER}-sic"),
        format!("{IDENTIFIER}-siw"),
    )
}

#[cfg(windows)]
fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(std::iter::once(0)).collect()
}

#[cfg(windows)]
/// Whether another rc-compress holds the single-instance mutex (running, or starting)
fn another_instance_exists(mutex_name: &str) -> bool {
    let name = wide(mutex_name);
    let handle = unsafe { OpenMutexW(SYNCHRONIZATION_SYNCHRONIZE, 0, name.as_ptr()) };
    if handle.is_null() {
        return false;
    }
    unsafe { CloseHandle(handle) };
    true
}

#[cfg(windows)]
fn window_exists(class_name: &str, window_name: &str) -> bool {
    let class_name = wide(class_name);
    let window_name = wide(window_name);
    !unsafe { FindWindowW(class_name.as_ptr(), window_name.as_ptr()) }.is_null()
}

/// Calls `is_ready` up to `tries` times, sleeping `interval` between; returns whether it became true
fn wait_until(tries: u32, interval: Duration, mut is_ready: impl FnMut() -> bool) -> bool {
    for attempt in 0..tries {
        if is_ready() {
            return true;
        }
        if attempt + 1 < tries {
            std::thread::sleep(interval);
        }
    }
    false
}

/// When another rc-compress is starting, waits (up to a second) until it can take this launch's files. Costs
/// nothing when no other instance exists, the usual case. If it never gets ready, the plugin handles the launch as
/// before (logging isn't set up yet, so nothing is logged here).
#[cfg(windows)]
pub fn wait_for_starting_instance() {
    let (mutex_name, class_name, window_name) = names();
    if !another_instance_exists(&mutex_name) {
        return;
    }
    wait_until(TRIES, TRY_INTERVAL, || window_exists(&class_name, &window_name));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_identifier_matches_the_app_config() {
        let config: serde_json::Value =
            serde_json::from_str(include_str!("../../tauri.conf.json")).unwrap();
        assert_eq!(config["identifier"], IDENTIFIER);
    }

    #[test]
    fn the_names_are_the_plugins() {
        let (mutex_name, class_name, window_name) = names();
        assert_eq!(mutex_name, "com.rccompress.app-sim");
        assert_eq!(class_name, "com.rccompress.app-sic");
        assert_eq!(window_name, "com.rccompress.app-siw");
    }

    #[test]
    fn waits_until_ready() {
        let mut calls = 0;
        let is_ready = wait_until(TRIES, Duration::ZERO, || {
            calls += 1;
            calls == 3
        });
        assert!(is_ready);
        assert_eq!(calls, 3);
    }

    #[test]
    fn gives_up_after_the_last_try() {
        let mut calls = 0;
        let is_ready = wait_until(TRIES, Duration::ZERO, || {
            calls += 1;
            false
        });
        assert!(!is_ready);
        assert_eq!(calls, TRIES);
    }
}
