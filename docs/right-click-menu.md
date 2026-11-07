# The right-click menu

rc-compress adds a **rc-compress** submenu to Windows File Explorer's right-click menu. The installer adds it and the
uninstaller removes it (`src-tauri/windows/hooks.nsh`); the app handles each item in
`src-tauri/src/sys/explorer_menu.rs`.

## Where it is

| System | Where |
|---|---|
| Windows 11 | Right-click a file → **Show more options** → rc-compress. The new, shorter menu only lists signed, packaged apps; rc-compress joins it once it is signed. |
| Windows 10 | Right-click a file → rc-compress. |
| macOS, Linux | Not yet: use the app window, the drop zone or the command line. The Right-click Menu settings page only shows on Windows. |

The menu is offered on every file; rc-compress skips what it can't compress. Several selected files are each
compressed.

## The items

| Item | What it does | Settings |
|---|---|---|
| **Quick Compress** | Compresses with your Quick Compress preset for that kind of media, or your own group's preset if the file's type is in one | Settings → Right-click Menu: a preset per kind of media, and **Your groups** (file types with their own preset, checked top to bottom) |
| **Compress by ▸ 10% … 90%** | Makes the file that much smaller (a target size). Videos and audio hit it closely; images get as close as their quality steps allow | The Quick Compress presets' other settings |
| **Fit to size ▸ 8 MB · 10 MB · 25 MB · Your size** | Makes the file fit within that size (8 and 10 MB for Discord, 25 MB for email) | **Your size**: Settings → Right-click Menu → Fit to size |
| **Remove metadata** | Copies the file without its location, camera and date details, without compressing it (videos and audio unchanged; images saved losslessly) | None |
| **Open in rc-compress…** | Opens the file in the app window, to choose every setting by hand | None |

## Where the copy goes

Next to the original, named `name (compressed).ext`, unless:
- the preset saves somewhere else (Settings → Output, or the preset's own output);
- a file name prefix is set: then `prefix` + the original name;
- **Replace original files** is on: then the copy keeps the original name and the original goes to the Recycle Bin.

A preset that asks where to save each time can't ask from the right-click menu: its files wait in the queue (the tray
icon) to be saved by hand, and Settings → Right-click Menu shows a warning next to it.

## In the background

When rc-compress is closed, a right-click item starts it without opening its window; progress shows on the tray icon
and in its queue. The first start after installing still shows the one-time "Preparing" window while the bundled
tools are unpacked.
