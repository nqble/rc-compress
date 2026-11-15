//! rc-compress in Finder's right-click menu (macOS): four Quick Actions, which Finder lists under
//! Quick Actions. Each one is an Automator workflow in `~/Library/Services` whose only step is a
//! shell script that runs the app's executable with a menu option, e.g. `compressO --compress-by=30
//! <files>`. That launch runs the command line, which hands the action to the app (see
//! `explorer_menu.rs`). The app writes them when Settings -> Right-click Menu's switch is on (the
//! Mac app has no installer) and deletes them when it is turned off.
//!
//! Finder offers no submenus here, so "Compress by…" and "Fit to size…" ask with a list.
//! Windows: the installer's Explorer entries instead (`src-tauri/windows/hooks.nsh`). Linux: not yet.
use std::path::{Path, PathBuf};

/// One Quick Action: its name in Finder & the shell script it runs (`APP` is the executable)
struct QuickAction {
    title: &'static str,
    script: &'static str,
}

/// The four items, covering the whole Windows menu. The options must match
/// `explorer_menu.rs`; the lists' choices match its `COMPRESS_BY_PERCENTS` & `FIT_SIZES_IN_MB`.
const QUICK_ACTIONS: [QuickAction; 4] = [
    QuickAction {
        title: "Compress with rc-compress",
        script: r#""$APP" --quick-compress "$@""#,
    },
    QuickAction {
        title: "Compress by…",
        script: r#"choice=$(osascript -e 'choose from list {"10%", "20%", "30%", "40%", "50%", "60%", "70%", "80%", "90%"} with title "rc-compress" with prompt "Make the files smaller by:" default items {"50%"}')
[ "$choice" = "false" ] && exit 0
"$APP" "--compress-by=${choice%\%}" "$@""#,
    },
    QuickAction {
        title: "Fit to size…",
        script: r#"choice=$(osascript -e 'choose from list {"8 MB (Discord)", "10 MB (Discord)", "25 MB (Email)", "Your size (set in the app)"} with title "rc-compress" with prompt "Make the files fit within:" default items {"10 MB (Discord)"}')
case "$choice" in
  "8 MB"*) size=8 ;;
  "10 MB"*) size=10 ;;
  "25 MB"*) size=25 ;;
  "Your size"*) size=custom ;;
  *) exit 0 ;;
esac
"$APP" "--fit-to=$size" "$@""#,
    },
    QuickAction {
        title: "Remove metadata",
        script: r#""$APP" --remove-metadata "$@""#,
    },
];

/// The ids Automator gives a workflow's step; any fixed ones do, as each workflow has one step
const STEP_UUIDS: [&str; 3] = [
    "6A1B0C1E-2D3F-4A5B-8C7D-9E0F1A2B3C4D",
    "7B2C1D2F-3E4A-4B6C-9D8E-0F1A2B3C4D5E",
    "8C3D2E3A-4F5B-4C7D-8E9F-1A2B3C4D5E6F",
];

/// A file of a Quick Action, relative to the Services folder
pub struct BundleFile {
    pub path: PathBuf,
    pub contents: String,
}

/// Every file of the four Quick Actions, for the app at `executable`
pub fn bundle_files(executable: &Path) -> Vec<BundleFile> {
    let quoted = shell_quote(&executable.display().to_string());
    QUICK_ACTIONS
        .iter()
        .flat_map(|action| {
            let script = format!("APP={quoted}\n{}\n", action.script);
            let folder = PathBuf::from(format!("{}.workflow", action.title)).join("Contents");
            [
                BundleFile {
                    path: folder.join("Info.plist"),
                    contents: info_plist(action.title),
                },
                BundleFile {
                    path: folder.join("document.wflow"),
                    contents: workflow(&script),
                },
            ]
        })
        .collect()
}

/// The Quick Actions' folders, relative to the Services folder (deleted when the switch is off)
pub fn bundle_folders() -> Vec<PathBuf> {
    QUICK_ACTIONS
        .iter()
        .map(|action| PathBuf::from(format!("{}.workflow", action.title)))
        .collect()
}

/// A word for the shell, in single quotes (a `'` inside closes, escapes & reopens them)
fn shell_quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', r"'\''"))
}

fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

const PLIST_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
"#;

/// Tells macOS this is a Quick Action for files & folders in Finder, named `title`. Finder lists a Quick Action only
/// when it has an icon & a background color, as Automator writes them; without them it is registered but never shown.
fn info_plist(title: &str) -> String {
    let title = xml_escape(title);
    format!(
        r#"{PLIST_HEADER}<dict>
	<key>NSServices</key>
	<array>
		<dict>
			<key>NSBackgroundColorName</key>
			<string>background</string>
			<key>NSIconName</key>
			<string>NSActionTemplate</string>
			<key>NSMenuItem</key>
			<dict>
				<key>default</key>
				<string>{title}</string>
			</dict>
			<key>NSMessage</key>
			<string>runWorkflowAsService</string>
			<key>NSRequiredContext</key>
			<dict>
				<key>NSApplicationIdentifier</key>
				<string>com.apple.finder</string>
			</dict>
			<key>NSSendFileTypes</key>
			<array>
				<string>public.item</string>
			</array>
		</dict>
	</array>
</dict>
</plist>
"#
    )
}

/// The workflow: one "Run Shell Script" step that is given the chosen files as arguments
fn workflow(script: &str) -> String {
    let script = xml_escape(script);
    let [input_uuid, output_uuid, step_uuid] = STEP_UUIDS;
    format!(
        r#"{PLIST_HEADER}<dict>
	<key>AMApplicationBuild</key>
	<string>523</string>
	<key>AMApplicationVersion</key>
	<string>2.10</string>
	<key>AMDocumentVersion</key>
	<string>2</string>
	<key>actions</key>
	<array>
		<dict>
			<key>action</key>
			<dict>
				<key>AMAccepts</key>
				<dict>
					<key>Container</key>
					<string>List</string>
					<key>Optional</key>
					<true/>
					<key>Types</key>
					<array>
						<string>com.apple.cocoa.string</string>
					</array>
				</dict>
				<key>AMActionVersion</key>
				<string>2.0.3</string>
				<key>AMApplication</key>
				<array>
					<string>Automator</string>
				</array>
				<key>AMParameterProperties</key>
				<dict>
					<key>COMMAND_STRING</key>
					<dict/>
					<key>CheckedForUserDefaultShell</key>
					<dict/>
					<key>inputMethod</key>
					<dict/>
					<key>shell</key>
					<dict/>
					<key>source</key>
					<dict/>
				</dict>
				<key>AMProvides</key>
				<dict>
					<key>Container</key>
					<string>List</string>
					<key>Types</key>
					<array>
						<string>com.apple.cocoa.string</string>
					</array>
				</dict>
				<key>ActionBundlePath</key>
				<string>/System/Library/Automator/Run Shell Script.action</string>
				<key>ActionName</key>
				<string>Run Shell Script</string>
				<key>ActionParameters</key>
				<dict>
					<key>COMMAND_STRING</key>
					<string>{script}</string>
					<key>CheckedForUserDefaultShell</key>
					<true/>
					<key>inputMethod</key>
					<integer>1</integer>
					<key>shell</key>
					<string>/bin/zsh</string>
					<key>source</key>
					<string></string>
				</dict>
				<key>BundleIdentifier</key>
				<string>com.apple.RunShellScript</string>
				<key>CFBundleVersion</key>
				<string>2.0.3</string>
				<key>CanShowSelectedItemsWhenRun</key>
				<false/>
				<key>CanShowWhenRun</key>
				<true/>
				<key>Category</key>
				<array>
					<string>AMCategoryUtilities</string>
				</array>
				<key>Class Name</key>
				<string>RunShellScriptAction</string>
				<key>InputUUID</key>
				<string>{input_uuid}</string>
				<key>Keywords</key>
				<array>
					<string>Shell</string>
					<string>Script</string>
				</array>
				<key>OutputUUID</key>
				<string>{output_uuid}</string>
				<key>UUID</key>
				<string>{step_uuid}</string>
				<key>UnlocalizedApplications</key>
				<array>
					<string>Automator</string>
				</array>
				<key>isViewVisible</key>
				<integer>1</integer>
			</dict>
			<key>isViewVisible</key>
			<integer>1</integer>
		</dict>
	</array>
	<key>connectors</key>
	<dict/>
	<key>workflowMetaData</key>
	<dict>
		<key>serviceApplicationBundleID</key>
		<string>com.apple.finder</string>
		<key>serviceApplicationPath</key>
		<string>/System/Library/CoreServices/Finder.app</string>
		<key>serviceInputTypeIdentifier</key>
		<string>com.apple.Automator.fileSystemObject</string>
		<key>serviceOutputTypeIdentifier</key>
		<string>com.apple.Automator.nothing</string>
		<key>serviceProcessesInput</key>
		<integer>0</integer>
		<key>workflowTypeIdentifier</key>
		<string>com.apple.Automator.servicesMenu</string>
	</dict>
</dict>
</plist>
"#
    )
}

/// Adds (`true`) or removes the Quick Actions; called at each start & when the switch changes,
/// so they follow the app when it is moved
#[tauri::command]
pub async fn set_finder_menu_enabled(enabled: bool) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || platform::apply(enabled))
        .await
        .map_err(|err| err.to_string())?
}

#[cfg(target_os = "macos")]
mod platform {
    use super::*;

    /// Makes macOS read the Services folder again, so a change shows without logging out
    const REFRESH_SERVICES: &str = "/System/Library/CoreServices/pbs";

    fn services_folder() -> Result<PathBuf, String> {
        let home = std::env::var_os("HOME").ok_or("No home folder")?;
        Ok(PathBuf::from(home).join("Library/Services"))
    }

    pub fn apply(enabled: bool) -> Result<(), String> {
        let folder = services_folder()?;
        let is_changed = if enabled {
            install(&folder)?
        } else {
            uninstall(&folder)?
        };
        if is_changed {
            let _ = std::process::Command::new(REFRESH_SERVICES)
                .arg("-update")
                .status();
        }
        Ok(())
    }

    /// Writes the files that are missing or out of date; returns whether any was
    fn install(folder: &Path) -> Result<bool, String> {
        let executable = std::env::current_exe().map_err(|err| err.to_string())?;
        let mut is_changed = false;
        for file in bundle_files(&executable) {
            let path = folder.join(&file.path);
            if std::fs::read_to_string(&path).ok().as_deref() == Some(file.contents.as_str()) {
                continue;
            }
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
            }
            std::fs::write(&path, &file.contents).map_err(|err| err.to_string())?;
            is_changed = true;
        }
        if is_changed {
            log::info!("[finder-menu] Quick Actions written");
        }
        Ok(is_changed)
    }

    /// Deletes rc-compress's own Quick Actions only; returns whether there were any
    fn uninstall(folder: &Path) -> Result<bool, String> {
        let mut is_changed = false;
        for bundle in bundle_folders() {
            let path = folder.join(bundle);
            if !path.exists() {
                continue;
            }
            std::fs::remove_dir_all(&path).map_err(|err| err.to_string())?;
            is_changed = true;
        }
        if is_changed {
            log::info!("[finder-menu] Quick Actions removed");
        }
        Ok(is_changed)
    }
}

#[cfg(not(target_os = "macos"))]
mod platform {
    pub fn apply(_enabled: bool) -> Result<(), String> {
        Err("Quick Actions are only on macOS".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn script_of(files: &[BundleFile], title: &str) -> String {
        let path = PathBuf::from(format!("{title}.workflow/Contents/document.wflow"));
        files
            .iter()
            .find(|file| file.path == path)
            .map(|file| file.contents.clone())
            .expect("the workflow of the action")
    }

    #[test]
    fn finder_lists_them_only_with_an_icon_and_a_background() {
        let files = bundle_files(Path::new("/a"));
        let info_plists: Vec<_> = files.iter().filter(|file| file.path.ends_with("Info.plist")).collect();
        assert_eq!(info_plists.len(), 4);
        for file in info_plists {
            assert!(file.contents.contains("<key>NSIconName</key>\n\t\t\t<string>NSActionTemplate</string>"));
            assert!(file.contents.contains("<key>NSBackgroundColorName</key>\n\t\t\t<string>background</string>"));
        }
    }

    #[test]
    fn four_quick_actions_of_two_files_each() {
        let files = bundle_files(Path::new(
            "/Applications/rc-compress.app/Contents/MacOS/compressO",
        ));
        assert_eq!(files.len(), 8);
        assert_eq!(bundle_folders().len(), 4);
        for folder in bundle_folders() {
            assert!(files.iter().any(|file| file.path.starts_with(&folder)));
        }
    }

    #[test]
    fn each_action_runs_the_app_with_its_menu_option() {
        let files = bundle_files(Path::new(
            "/Applications/rc-compress.app/Contents/MacOS/compressO",
        ));
        let expected = [
            ("Compress with rc-compress", "--quick-compress"),
            ("Compress by…", "--compress-by="),
            ("Fit to size…", "--fit-to="),
            ("Remove metadata", "--remove-metadata"),
        ];
        for (title, option) in expected {
            let workflow = script_of(&files, title);
            assert!(
                workflow.contains("APP='/Applications/rc-compress.app/Contents/MacOS/compressO'")
            );
            assert!(workflow.contains(option), "{title}");
            assert!(
                workflow.contains("&quot;$@&quot;"),
                "{title} passes the files"
            );
        }
    }

    #[test]
    fn the_lists_offer_what_the_app_accepts() {
        let files = bundle_files(Path::new("/a"));
        let compress_by = script_of(&files, "Compress by…");
        for percent in crate::sys::explorer_menu::COMPRESS_BY_PERCENTS {
            assert!(
                compress_by.contains(&format!("&quot;{percent}%&quot;")),
                "{percent}%"
            );
        }
        let fit_to = script_of(&files, "Fit to size…");
        for megabytes in crate::sys::explorer_menu::FIT_SIZES_IN_MB {
            assert!(
                fit_to.contains(&format!("size={megabytes} ;;")),
                "{megabytes} MB"
            );
        }
        assert!(fit_to.contains("size=custom"));
    }

    #[test]
    fn an_app_path_with_quotes_stays_one_word() {
        assert_eq!(
            shell_quote("/Apps/Jo's/compressO"),
            r"'/Apps/Jo'\''s/compressO'"
        );
        let files = bundle_files(Path::new("/Apps/A & B/compressO"));
        assert!(script_of(&files, "Remove metadata").contains("APP='/Apps/A &amp; B/compressO'"));
    }
}
