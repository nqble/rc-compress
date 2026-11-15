//! The switch "Show in the file manager's menu" (Settings -> Right-click Menu): adds or removes
//! rc-compress's entries in the file manager's right-click menu, at each start (so they follow the
//! app when it is moved) & when the switch changes. macOS: Finder's Quick Actions (`finder_menu.rs`);
//! Linux: Dolphin, Files & Nemo (`linux_menu.rs`); Windows: nothing to do, the installer adds them
//! (`src-tauri/windows/hooks.nsh`) and the switch isn't shown.

#[tauri::command]
pub async fn set_file_manager_menu_enabled(enabled: bool) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || apply(enabled))
        .await
        .map_err(|err| err.to_string())?
}

#[cfg(target_os = "macos")]
fn apply(enabled: bool) -> Result<(), String> {
    super::finder_menu::apply(enabled)
}

#[cfg(target_os = "linux")]
fn apply(enabled: bool) -> Result<(), String> {
    super::linux_menu::apply(enabled)
}

#[cfg(windows)]
fn apply(_enabled: bool) -> Result<(), String> {
    Ok(())
}
