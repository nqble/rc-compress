# rc-compress: how we build it

The standing brief for every Claude Code session working on rc-compress. **Code style and project layout are in
[AGENTS.md](AGENTS.md)** (inherited from compressO); this file adds how we work. When the two disagree, this file
wins. When something is unclear, ask the maintainer before guessing.

rc-compress ("right-click compress") is a fork of [compressO](https://github.com/codeforreal1/compressO) under the
AGPL-3.0: it makes videos, images, audio and PDFs smaller, entirely on the user's computer, with a right-click
"Compress" action in the file manager (Windows first, then macOS and Linux).

## Commands

- Run the app: `pnpm tauri:dev`. Web page only: `pnpm vite:dev`.
- Before every commit: Biome as in `checks.yml` (`pnpm biome check --write` on the files you changed), `pnpm tsc:check`, `pnpm i18n:check`, `pnpm test:drop-queue`,
  `pnpm test:audio-target-size`, `pnpm test:claude-hooks`, `cargo test --manifest-path src-tauri/Cargo.toml` and
  `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets`. CI runs the same (`.github/workflows/checks.yml`).
- A Windows installer from a branch: push to a `claude/**` or `test/**` branch; `.github/workflows/test-build.yml`
  keeps it as a workflow artifact. Releases: `.github/workflows/release-build.yml` (the maintainer's word first).

## 1. Principles, in priority order

1. **Private and offline.** Files never leave the computer. The only network use is the app's own update check.
2. **Readable by a newcomer.** Obvious code over clever code; a file should make sense read top to bottom.
3. **Fast and light.** No work while idle; measure before and after a change that could cost speed or memory.
4. **Stay close to upstream.** Keep compressO's structure and style so its improvements still merge cleanly: add new
   features in new files where you can, and change upstream files only as much as needed.

## 2. Code and comments

- Follow AGENTS.md (Biome formatting, TypeScript, React, Valtio, Tauri commands, every text in all 16 languages).
- One step per line, short functions (~40 lines), early returns, descriptive names, named constants, no dead or
  commented-out code. Search for an existing helper before writing one.
- Comments say **why**, not what; a new file starts with a short header saying what it is for. Keep comments and docs
  true in the same commit as the change.
- **Every system:** say how each feature behaves on Windows, macOS and Linux; one that works on one system only hides
  or explains itself elsewhere instead of failing.

## 3. Tests

- Every bug fix comes with a test that fails without it; every feature comes with tests.
- If a test fails, stop and report it. Never delete or weaken a test to make it pass.

## 4. Commits and pull requests

- **Small, focused commits** in [Conventional Commits](https://www.conventionalcommits.org): `type(scope): summary`,
  lower case, no full stop, under ~72 characters. Types: `feat`, `fix`, `perf`, `refactor`, `test`, `docs`, `style`,
  `build`, `ci`, `chore`. `.claude/hooks/commit-format.mjs` refuses other messages.
- Work on a branch and open a pull request; `.claude/hooks/no-push-to-main.mjs` refuses pushes to `main` and force
  pushes.
- No AI co-author or "Generated with" lines in commits, pull requests or code.
- **Ask the maintainer before:** pushing to `main`, force-pushing, rewriting history, tags or releases, posting on
  GitHub outside your own pull requests, repository settings, adding any dependency, a big refactor, or merging
  changes from upstream compressO.
- **Licenses:** everything stays AGPL-3.0-compatible. Keep compressO's credit, `LICENSE`, `LICENSES/` and
  `THIRD_PARTY_NOTICES.md`; a new bundled tool or library adds its notice there in the same commit.
- **Never commit secrets** (the updater signing key, tokens).

## 5. Before the first release

The name (rc-compress), identifier (`com.rccompress.app`) and updater endpoint (`nqble/rc-compress` releases) are
rc-compress's own. Still compressO's: the updater's **public key** in `src-tauri/tauri.conf.json` (replace it with
rc-compress's own before the first release; until then no update can install, since none is signed with it), the
icon (kept for now), and internal names (the `compressO` binary and crate, the `compresso_*` tool files), left as
they are so upstream changes keep merging.

## Skills

In `.claude/skills/`: verification-before-completion before calling anything done, systematic-debugging for any bug,
requesting-code-review and receiving-code-review for a second look.
