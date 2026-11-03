<div align="center">
  <img width="150" height="150" src="public/app-icon-dark.png" alt="rc-compress logo" />
  <h1>rc-compress</h1>
  <p>Right-click compress: make videos, images, audio and PDFs smaller, entirely on your own computer.</p>
  <p>Free and open source, for <strong>Windows</strong>, <strong>macOS</strong> and <strong>Linux</strong>.</p>
</div>

> **Not released yet.** rc-compress is being built. Until its first release (0.1.0), use
> [compressO](https://github.com/codeforreal1/compressO), the app it is based on.

## What it does

- Shrinks videos, images and audio (and PDFs, coming in the first release), with full control over format,
  quality and size for people who want it.
- Nothing is uploaded: every file is compressed on your computer, and the only network use is checking for updates.
- **Right-click "Compress"** in your file manager (planned for the first release, Windows first):

  ```
  rc-compress ▸  Quick Compress
                 Compress by 10% … 90%
                 Fit to size ▸  8 MB · 10 MB · 25 MB · your own size
                 Remove metadata
                 Open in rc-compress…
  ```

  The compressed copy is saved next to the original; the original is never changed.

## Based on compressO

rc-compress is a fork of [compressO](https://github.com/codeforreal1/compressO) by
[Code For Real](https://codeforreal.com), which does the compressing (with FFmpeg and other open-source tools; see
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)). Thank you to its authors.

## License

[AGPL-3.0](LICENSE), like compressO.

## Contributing

How the project is built and checked: [CLAUDE.md](CLAUDE.md) and [AGENTS.md](AGENTS.md). Run the app with
`pnpm install` and `pnpm tauri:dev`.
