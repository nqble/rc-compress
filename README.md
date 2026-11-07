<div align="center">
  <img width="150" height="150" src="public/app-icon-dark.png" alt="rc-compress logo" />
  <h1>rc-compress</h1>
  <p>Right-click compress: make videos, images and audio smaller, entirely on your own computer.</p>
  <p>Free and open source, for <strong>Windows</strong>, <strong>macOS</strong> and <strong>Linux</strong>.</p>
</div>

> **Test versions (0.1.x).** rc-compress is in early development. It works, but expect rough edges; installed copies
> update themselves (Settings → About → Check for updates).

## Download

From the [latest release](https://github.com/nqble/rc-compress/releases/latest):

| Your system | File |
|---|---|
| Windows 10/11 | `rc-compress_<version>_x64.exe` |
| Mac with Apple Silicon (M1 or newer) | `rc-compress_<version>_aarch64.dmg` |
| Mac with Intel | `rc-compress_<version>_x64.dmg` |
| Linux (Ubuntu, Debian) | `rc-compress_<version>_amd64.deb` |
| Linux (any) | `rc-compress_<version>_amd64.AppImage` |

The downloads aren't signed yet: Windows may show "Windows protected your PC" (More info → Run anyway), and macOS may
say the app is damaged or from an unidentified developer (right-click the app → Open).

## What it does

- **Shrinks videos, images and audio**, with full control over format, quality, size, resolution and more, or one
  click with a preset. HDR is kept (or removed, or boosted), and many files can be compressed at once.
- **Nothing is uploaded:** every file is compressed on your computer. The only network use is checking for updates.
- **Right-click menu (Windows):** in File Explorer, right-click a file → Show more options → **rc-compress**:

  ```
  rc-compress ▸  Quick Compress
                 Compress by ▸  10% … 90%
                 Fit to size ▸  8 MB · 10 MB · 25 MB · Your size
                 Remove metadata
                 Open in rc-compress…
  ```

  The file is compressed in the background and the copy is saved next to the original as `name (compressed).ext`;
  the original is never changed. How each item works: [docs/right-click-menu.md](docs/right-click-menu.md).
- **Coming:** PDFs, and the right-click menu on macOS and Linux.

## Based on compressO

rc-compress is a fork of [compressO](https://github.com/codeforreal1/compressO) by
[Code For Real](https://codeforreal.com), which does the compressing (with FFmpeg and other open-source tools; see
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)). Thank you to its authors. Everything compressO does, rc-compress
does too, including its command line tool (Settings → CLI).

## License

[AGPL-3.0](LICENSE), like compressO.

## Contributing

How the project is built and checked: [CLAUDE.md](CLAUDE.md) and [AGENTS.md](AGENTS.md). Run the app with
`pnpm install` and `pnpm tauri:dev`. More in [docs/](docs/).
