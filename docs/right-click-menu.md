# The right-click menu

rc-compress adds a **rc-compress** submenu to Windows File Explorer's right-click menu, four Quick Actions to
macOS Finder's, and entries to the Linux file managers Dolphin, Files and Nemo. On Windows the installer adds it and
the uninstaller removes it (`src-tauri/windows/hooks.nsh`); on macOS and Linux the app adds them itself
(`src-tauri/src/sys/finder_menu.rs`, `src-tauri/src/sys/linux_menu.rs`). The app handles each item in
`src-tauri/src/sys/explorer_menu.rs`.

## Where it is

| System | Where |
|---|---|
| Windows 11 | Right-click a file → **Show more options** → rc-compress. The new, shorter menu only lists signed, packaged apps; rc-compress joins it once it is signed. |
| Windows 10 | Right-click a file → rc-compress. |
| macOS | Right-click a file → **Quick Actions** → one of the four below (see "On macOS"). |
| Linux | Dolphin (KDE): right-click → rc-compress. Files (GNOME): right-click → **Scripts** → rc-compress. Nemo (Cinnamon): four items in the right-click menu. Thunar (Xfce) and others: not yet (see "On Linux"). |

The menu is offered on every file; rc-compress compresses videos, images, audio and PDFs, and quietly skips
anything else (a `.txt`, a `.zip`). When none of the chosen files can be compressed, the tray panel opens with a note
saying so. Several selected files are each compressed.

**Folders:** the same menu is on folders. A folder's media is compressed, subfolders included, each file by its own
kind (its preset or PDF quality), and each copy is saved next to its original; the folder's other files are skipped.

## The items

| Item | What it does | Settings |
|---|---|---|
| **Quick Compress** | Compresses with your Quick Compress preset for that kind of media, or your own group's preset if the file's type is in one | Settings → Right-click Menu: a preset per kind of media, and **Your groups** (file types with their own preset, checked top to bottom) |
| **Compress by ▸ 10% … 90%** | Makes the file that much smaller (a target size). Videos and audio hit it closely; images get as close as their quality steps allow | The Quick Compress presets' other settings |
| **Fit to size ▸ 8 MB · 10 MB · 25 MB · Your size** | Makes the file fit within that size (8 and 10 MB for Discord, 25 MB for email) | **Your size**: Settings → Right-click Menu → Fit to size |
| **Remove metadata** | Copies the file without its location, camera and date details, without compressing it (videos and audio unchanged; images saved losslessly) | None |
| **Open in rc-compress…** | Opens the file in the app window, to choose every setting by hand | None |

## On macOS

Finder has no submenus for Quick Actions, so the menu has four items, and two of them ask with a list:

| Quick Action | Same as |
|---|---|
| **Compress with rc-compress** | Quick Compress |
| **Compress by…** | Compress by: a list asks 10% … 90% |
| **Fit to size…** | Fit to size: a list asks 8 MB, 10 MB, 25 MB or Your size |
| **Remove metadata** | Remove metadata |

There is no **Open in rc-compress…** item: Finder's **Open With** does that. The switch **Show in Finder's menu**
(Settings → Right-click Menu, on by default) adds the Quick Actions to `~/Library/Services` and removes them when
turned off; the app rewrites them at each start when they changed, so they follow it when it is moved. macOS lists
them unticked: turn each one on once, in Finder → right-click a file → **Quick Actions** → **Customize…**, and again
after the switch has been turned off and on (that writes them anew). Turn the switch off before
deleting the app, or the Quick Actions stay and do nothing (delete them from `~/Library/Services` by hand). Each
Quick Action runs the app's executable with the menu option (`compressO --compress-by=30 <files>`), which hands it to
the app through the command line's connection, starting the app in the background when it isn't running.

## On Linux

The switch **Show in the file manager's menu** (Settings → Right-click Menu, on by default) writes the entries into
your data folder (`~/.local/share`, or `$XDG_DATA_HOME`) and deletes them when turned off; the app rewrites them at
each start when they changed, so an AppImage that is moved keeps working. Turn the switch off before uninstalling.

| File manager | Where the entries are | What it shows |
|---|---|---|
| Dolphin (KDE) | `kio/servicemenus/rc-compress.desktop` | An **rc-compress** submenu with every item (Compress by and Fit to size as one item per choice: a service menu has one level of submenu only) |
| Files (GNOME, Nautilus) | `nautilus/scripts/rc-compress/` | **Scripts → rc-compress**, with the same submenus as on Windows |
| Nemo (Cinnamon) | `nemo/actions/rc-compress-*.nemo_action` | Four items, as Nemo has no submenus: Compress with rc-compress, Compress by 50%, Fit to 10 MB, Remove metadata |

Thunar (Xfce) keeps its custom actions in one file it shares with the ones you made yourself, so rc-compress doesn't
write to it. There is no **Open in rc-compress…** item: the file manager's **Open With** does that.

Linux has no tray icon, so a right-click item shows a small notice in the bottom-right corner of the screen: how many
files are being compressed and how far it is, then how many were compressed (or that none of them can be), and it
closes itself a few seconds later. **Open rc-compress** on it shows the main window with the queue. The main window
compresses meanwhile, hidden, and an app that a right-click started quits when the notice closes. When the main window
is already open, it shows the queue itself and no notice appears.

## PDFs

PDFs are compressed with the bundled Ghostscript (nothing to install); text stays sharp and selectable, and pictures
inside get a lower resolution. **Quick Compress** uses your **PDF quality** (Settings → Right-click Menu: Small,
Medium or High). **Compress by** and **Fit to size** try High, then Medium, then Small, and keep the first that is
small enough (or the smallest). **Remove metadata** compresses at High and empties the document's information (title, author and the
like).

## Where the copy goes

Next to the original, named `name (compressed).ext`, unless:
- the preset saves somewhere else (Settings → Output, or the preset's own output);
- a file name prefix is set: then `prefix` + the original name;
- **Replace original files** is on: then the copy keeps the original name and the original goes to the Recycle Bin (the Trash on macOS).

A preset that asks where to save each time can't ask from the right-click menu: its files wait in the queue (the tray
icon) to be saved by hand, and Settings → Right-click Menu shows a warning next to it.

## In the background

When rc-compress is closed, a right-click item starts it without opening its window; progress shows on the tray icon (the menu bar icon on macOS; on Linux, which has none, in a small notice)
and in its queue. The first start after installing still shows the one-time "Preparing" window while the bundled
tools are unpacked.
