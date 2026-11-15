//! rc-compress in the Linux file managers' right-click menus. Each entry runs the app with a menu
//! option & the chosen files (`rc-compress --compress-by=30 <files>`); a launch while the app runs
//! reaches it as a second instance, as on Windows (see `explorer_menu.rs` & `on_second_instance`).
//! The `.deb` and the AppImage can't write into a user's home folder, so the app writes the entries
//! when Settings -> Right-click Menu's switch is on & deletes them when it is turned off:
//! - Dolphin (KDE): a service menu, the whole menu in a "rc-compress" submenu (one level only)
//! - Files (GNOME, Nautilus): scripts in `Scripts > rc-compress`, folders giving the submenus
//! - Nemo (Cinnamon): four actions, as it has no submenus
//!
//! Thunar (Xfce) keeps its custom actions in one file shared with the user's own, so it isn't
//! written to. macOS: `finder_menu.rs`; Windows: the installer (`src-tauri/windows/hooks.nsh`).
use std::path::{Path, PathBuf};

use super::explorer_menu::{
    COMPRESS_BY_ARG, COMPRESS_BY_PERCENTS, FIT_SIZES_IN_MB, FIT_TO_ARG, QUICK_COMPRESS_ARG,
    REMOVE_METADATA_ARG,
};

/// Where each file manager reads its entries, relative to the user's data folder
/// (`~/.local/share`)
const KDE_SERVICE_MENU: &str = "kio/servicemenus/rc-compress.desktop";
const NAUTILUS_SCRIPTS: &str = "nautilus/scripts/rc-compress";
const NEMO_ACTIONS: &str = "nemo/actions";

/// Nemo's four items: the ones the Windows menu's submenus would hide
const NEMO_COMPRESS_BY_PERCENT: u8 = 50;
const NEMO_FIT_SIZE_IN_MB: u16 = 10;

/// One item of the menu: its name & the option it launches the app with
struct MenuItem {
    title: String,
    option: String,
}

fn item(title: impl Into<String>, option: impl Into<String>) -> MenuItem {
    MenuItem {
        title: title.into(),
        option: option.into(),
    }
}

fn compress_by(percent: u8) -> String {
    format!("{COMPRESS_BY_ARG}{percent}")
}

fn fit_to(size: &str) -> String {
    format!("{FIT_TO_ARG}{size}")
}

/// The sizes of "Fit to size", with their names as on Windows (`hooks.nsh`)
fn fit_sizes() -> Vec<MenuItem> {
    let mut sizes: Vec<MenuItem> = FIT_SIZES_IN_MB
        .iter()
        .map(|megabytes| {
            let purpose = if *megabytes < 25 { "Discord" } else { "Email" };
            item(format!("{megabytes} MB ({purpose})"), fit_to(&megabytes.to_string()))
        })
        .collect();
    sizes.push(item("Your size (set in the app)", fit_to("custom")));
    sizes
}

/// A file of the entries, relative to the user's data folder
pub struct EntryFile {
    pub path: PathBuf,
    pub contents: String,
    /// KDE & Nautilus run only entries that are executable
    pub is_executable: bool,
}

/// Every file of the entries, for the app at `executable`
pub fn entry_files(executable: &Path) -> Vec<EntryFile> {
    let executable = executable.display().to_string();
    let mut files = vec![kde_service_menu(&executable)];
    files.extend(nautilus_scripts(&executable));
    files.extend(nemo_actions(&executable));
    files
}

/// What the switch deletes, relative to the user's data folder: rc-compress's own entries only
pub fn entry_paths() -> Vec<PathBuf> {
    let mut paths = vec![PathBuf::from(KDE_SERVICE_MENU), PathBuf::from(NAUTILUS_SCRIPTS)];
    paths.extend(nemo_actions("").into_iter().map(|file| file.path));
    paths
}

/// A word for the shell (and Nemo's `Exec`), in single quotes
fn shell_quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', r"'\''"))
}

/// A program path in a `.desktop` file's `Exec`: in double quotes, with `"`, `` ` ``, `$` & `\`
/// escaped, then every `\` escaped again, as the Desktop Entry specification asks
fn desktop_quote(text: &str) -> String {
    let mut quoted = String::new();
    for character in text.chars() {
        if matches!(character, '"' | '`' | '$' | '\\') {
            quoted.push('\\');
        }
        quoted.push(character);
    }
    format!("\"{}\"", quoted.replace('\\', r"\\"))
}

fn kde_service_menu(executable: &str) -> EntryFile {
    let mut items = vec![item("Quick Compress", QUICK_COMPRESS_ARG)];
    for percent in COMPRESS_BY_PERCENTS {
        items.push(item(format!("Compress by {percent}%"), compress_by(percent)));
    }
    for size in fit_sizes() {
        items.push(item(format!("Fit to {}", size.title), size.option));
    }
    items.push(item("Remove metadata", REMOVE_METADATA_ARG));

    let ids: Vec<String> = (1..=items.len()).map(|number| format!("item{number}")).collect();
    let mut contents = format!(
        "[Desktop Entry]\nType=Service\nMimeType=all/all;\nActions={};\nX-KDE-Submenu=rc-compress\n",
        ids.join(";")
    );
    let program = desktop_quote(executable);
    for (id, item) in ids.iter().zip(&items) {
        contents.push_str(&format!(
            "\n[Desktop Action {id}]\nName={}\nExec={program} {} %F\n",
            item.title, item.option
        ));
    }
    EntryFile {
        path: PathBuf::from(KDE_SERVICE_MENU),
        contents,
        is_executable: true,
    }
}

/// A script Nautilus runs with the chosen files as its arguments
fn nautilus_script(path: PathBuf, executable: &str, option: &str) -> EntryFile {
    EntryFile {
        path: Path::new(NAUTILUS_SCRIPTS).join(path),
        contents: format!("#!/bin/sh\nexec {} {option} \"$@\"\n", shell_quote(executable)),
        is_executable: true,
    }
}

fn nautilus_scripts(executable: &str) -> Vec<EntryFile> {
    // Nautilus lists scripts by name, so a number keeps the Windows order
    let mut scripts = vec![nautilus_script("1 Quick Compress".into(), executable, QUICK_COMPRESS_ARG)];
    for percent in COMPRESS_BY_PERCENTS {
        let path = Path::new("2 Compress by").join(format!("{percent}%"));
        scripts.push(nautilus_script(path, executable, &compress_by(percent)));
    }
    for (number, size) in fit_sizes().into_iter().enumerate() {
        let path = Path::new("3 Fit to size").join(format!("{} {}", number + 1, size.title));
        scripts.push(nautilus_script(path, executable, &size.option));
    }
    scripts.push(nautilus_script("4 Remove metadata".into(), executable, REMOVE_METADATA_ARG));
    scripts
}

fn nemo_actions(executable: &str) -> Vec<EntryFile> {
    let items = [
        ("quick", item("Compress with rc-compress", QUICK_COMPRESS_ARG)),
        (
            "compress-by",
            item(format!("Compress by {NEMO_COMPRESS_BY_PERCENT}% (rc-compress)"), compress_by(NEMO_COMPRESS_BY_PERCENT)),
        ),
        (
            "fit-to",
            item(format!("Fit to {NEMO_FIT_SIZE_IN_MB} MB (rc-compress)"), fit_to(&NEMO_FIT_SIZE_IN_MB.to_string())),
        ),
        ("metadata", item("Remove metadata (rc-compress)", REMOVE_METADATA_ARG)),
    ];
    let program = shell_quote(executable);
    items
        .into_iter()
        .map(|(id, item)| EntryFile {
            path: Path::new(NEMO_ACTIONS).join(format!("rc-compress-{id}.nemo_action")),
            contents: format!(
                "[Nemo Action]\nName={}\nExec={program} {} %F\nSelection=notnone\nExtensions=any;\n",
                item.title, item.option
            ),
            is_executable: false,
        })
        .collect()
}

/// The executable the entries run: an AppImage's own file, not the folder it is mounted at while
/// it runs (gone once it quits)
fn launched_executable() -> Result<PathBuf, String> {
    if let Some(appimage) = std::env::var_os("APPIMAGE") {
        return Ok(PathBuf::from(appimage));
    }
    std::env::current_exe().map_err(|err| err.to_string())
}

/// The user's data folder (`$XDG_DATA_HOME`, or `~/.local/share`)
fn data_folder() -> Result<PathBuf, String> {
    if let Some(folder) = std::env::var_os("XDG_DATA_HOME").filter(|folder| !folder.is_empty()) {
        return Ok(PathBuf::from(folder));
    }
    let home = std::env::var_os("HOME").ok_or("No home folder")?;
    Ok(PathBuf::from(home).join(".local/share"))
}

/// Adds (`true`) or removes the entries
pub fn apply(enabled: bool) -> Result<(), String> {
    let folder = data_folder()?;
    if enabled {
        write_entries(&folder, &entry_files(&launched_executable()?))
    } else {
        remove_entries(&folder)
    }
}

/// Writes the files that are missing or out of date
fn write_entries(folder: &Path, files: &[EntryFile]) -> Result<(), String> {
    use std::os::unix::fs::PermissionsExt;

    // The scripts of an older menu are left behind otherwise
    let scripts = folder.join(NAUTILUS_SCRIPTS);
    for file in files {
        let path = folder.join(&file.path);
        if std::fs::read_to_string(&path).ok().as_deref() == Some(file.contents.as_str()) {
            continue;
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
        }
        std::fs::write(&path, &file.contents).map_err(|err| err.to_string())?;
        if file.is_executable {
            let permissions = std::fs::Permissions::from_mode(0o755);
            std::fs::set_permissions(&path, permissions).map_err(|err| err.to_string())?;
        }
        log::info!("[linux-menu] Wrote {}", file.path.display());
    }
    let _ = scripts;
    Ok(())
}

fn remove_entries(folder: &Path) -> Result<(), String> {
    for entry in entry_paths() {
        let path = folder.join(&entry);
        let result = if path.is_dir() {
            std::fs::remove_dir_all(&path)
        } else if path.exists() {
            std::fs::remove_file(&path)
        } else {
            continue;
        };
        result.map_err(|err| err.to_string())?;
        log::info!("[linux-menu] Removed {}", entry.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXECUTABLE: &str = "/usr/bin/compressO";

    fn contents_of(files: &[EntryFile], path: &str) -> String {
        files
            .iter()
            .find(|file| file.path == Path::new(path))
            .map(|file| file.contents.clone())
            .unwrap_or_else(|| panic!("no {path}"))
    }

    #[test]
    fn kde_gets_the_whole_menu_in_one_submenu() {
        let menu = contents_of(&entry_files(Path::new(EXECUTABLE)), KDE_SERVICE_MENU);
        assert!(menu.contains("X-KDE-Submenu=rc-compress"));
        assert!(menu.contains("Exec=\"/usr/bin/compressO\" --quick-compress %F"));
        for percent in COMPRESS_BY_PERCENTS {
            assert!(menu.contains(&format!("--compress-by={percent} %F")), "{percent}%");
        }
        for megabytes in FIT_SIZES_IN_MB {
            assert!(menu.contains(&format!("--fit-to={megabytes} %F")), "{megabytes} MB");
        }
        assert!(menu.contains("--fit-to=custom %F"));
        assert!(menu.contains("--remove-metadata %F"));
        // Every action listed is defined, & the other way round
        let actions = menu.lines().find_map(|line| line.strip_prefix("Actions=")).unwrap();
        let listed = actions.trim_end_matches(';').split(';').count();
        assert_eq!(listed, menu.matches("[Desktop Action ").count());
    }

    #[test]
    fn nautilus_scripts_make_the_windows_submenus() {
        let files = entry_files(Path::new(EXECUTABLE));
        let quick = contents_of(&files, "nautilus/scripts/rc-compress/1 Quick Compress");
        assert_eq!(quick, "#!/bin/sh\nexec '/usr/bin/compressO' --quick-compress \"$@\"\n");
        let half = contents_of(&files, "nautilus/scripts/rc-compress/2 Compress by/50%");
        assert!(half.contains("--compress-by=50 \"$@\""));
        let custom = contents_of(&files, "nautilus/scripts/rc-compress/3 Fit to size/4 Your size (set in the app)");
        assert!(custom.contains("--fit-to=custom"));
        assert!(files
            .iter()
            .filter(|file| file.path.starts_with(NAUTILUS_SCRIPTS))
            .all(|file| file.is_executable));
    }

    #[test]
    fn nemo_gets_four_actions() {
        let files = entry_files(Path::new(EXECUTABLE));
        let nemo: Vec<_> = files.iter().filter(|file| file.path.starts_with(NEMO_ACTIONS)).collect();
        assert_eq!(nemo.len(), 4);
        assert!(nemo.iter().all(|file| file.contents.contains("Exec='/usr/bin/compressO' --")));
        assert!(nemo.iter().any(|file| file.contents.contains("--compress-by=50 %F")));
        assert!(nemo.iter().any(|file| file.contents.contains("--fit-to=10 %F")));
    }

    #[test]
    fn the_switch_removes_exactly_what_it_wrote() {
        let files = entry_files(Path::new(EXECUTABLE));
        let removed = entry_paths();
        for file in &files {
            assert!(removed.iter().any(|path| file.path.starts_with(path)), "{}", file.path.display());
        }
        // Nothing outside rc-compress's own entries: never the whole scripts or actions folder
        assert!(!removed.contains(&PathBuf::from("nautilus/scripts")));
        assert!(!removed.contains(&PathBuf::from(NEMO_ACTIONS)));
    }

    #[test]
    fn paths_with_quotes_and_spaces_stay_one_word() {
        assert_eq!(shell_quote("/home/jo/Jo's Apps/rc.AppImage"), r"'/home/jo/Jo'\''s Apps/rc.AppImage'");
        assert_eq!(desktop_quote("/opt/my apps/rc"), "\"/opt/my apps/rc\"");
        assert_eq!(desktop_quote("/opt/$x\"y"), r#""/opt/\\$x\\"y""#);
    }

    #[test]
    fn writing_twice_changes_nothing_and_removing_leaves_other_entries() {
        let folder = std::env::temp_dir().join(format!("rc-compress-linux-menu-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        let others = folder.join("nautilus/scripts/My own script");
        std::fs::create_dir_all(others.parent().unwrap()).unwrap();
        std::fs::write(&others, "#!/bin/sh\n").unwrap();

        let files = entry_files(Path::new(EXECUTABLE));
        write_entries(&folder, &files).unwrap();
        write_entries(&folder, &files).unwrap();
        for file in &files {
            assert_eq!(std::fs::read_to_string(folder.join(&file.path)).unwrap(), file.contents);
        }
        remove_entries(&folder).unwrap();
        for file in &files {
            assert!(!folder.join(&file.path).exists(), "{}", file.path.display());
        }
        assert!(others.exists());
        let _ = std::fs::remove_dir_all(&folder);
    }
}
