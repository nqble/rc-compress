# Ghostscript (PDF compression)

rc-compress makes PDFs smaller with [Ghostscript](https://ghostscript.com/) 10.08.0 (AGPL-3.0, like rc-compress),
bundled like FFmpeg: `src-tauri/bin/compresso_ghostscript-<target>` is packed into the offline tool bundle
([tool-bundles.md](tool-bundles.md)) and run by the app as the tool `ghostscript`. Nothing is downloaded at runtime.

## Where the executables come from

`.github/workflows/build-ghostscript.yml` makes them and commits them to the branch it runs on:

| Target | How |
|---|---|
| Linux x64, macOS Apple Silicon, macOS Intel | Built from the source archive `ghostscript-10.08.0.tar.gz` (SHA-256 `caf199e3f233f1290b27d0972d636f66c303355f2353309b7bfddf1edda06b3d`), on Ubuntu 22.04 and macOS (deployment target 11.0), with only what PDF compression needs: no printers (CUPS), windows (GTK, X11), system fonts, D-Bus or OCR. Ghostscript's own copies of zlib, libpng, libjpeg, FreeType, lcms2, OpenJPEG and jbig2dec are built in, so the executable needs only the system's C library. |
| Windows x64 | Artifex's official build, from `gs10080w64.exe` (SHA-256 `52a91b8bf09298788d7a57b9206127026c23eacd75405f0a131e26dc381dce50`): `gswin64c.exe` (renamed) and `gsdll64.dll`, kept in `src-tauri/bin/ghostscript-windows/` and bundled next to it. |

Each download is checked against its pinned SHA-256, and each executable must turn a sample PDF into a smaller one
before it is committed. To update Ghostscript, change the version, tag and checksums in the workflow and here, and run it
on a work branch.

## Source and license

The license is in `LICENSES/ghostscript/` (shipped with the app) and the notice in `THIRD_PARTY_NOTICES.md`. The exact
source archive is published by Artifex at
https://github.com/ArtifexSoftware/ghostpdl-downloads/releases/tag/gs10080, as AGPL requires.
