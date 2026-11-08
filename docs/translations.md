# Translations

rc-compress's interface is in 16 languages (`src/i18n/locales/`, one JSON file each; `pnpm i18n:check` checks that every
language has every text and keeps its placeholders).

## Where they come from

- **compressO's texts** (most of them) come from upstream with each sync and stay as compressO made them; rc-compress
  only replaced the app's name.
- **rc-compress's own texts** (the right-click menu page, Quick Compress groups, PDF quality, the tray note, the PDF
  queue note) are written in English, then translated from English by
  [Index-Translate](https://github.com/bilibili/Index-Translate) (bilibili's translation model, Apache-2.0), with a
  glossary so a name like "Quick Compress" is translated the same way everywhere. They were first written by an AI
  coding assistant and replaced this way on 2026-11-08.

Machine translations can be wrong in ways a native speaker notices at once. A correction is welcome as a pull request
changing the line in that language's file.

## Adding a text

Write it in `en.json`, then add it to every other language (`pnpm i18n:check` lists what is missing), translated from
the English the same way, with the glossary terms that language already uses.
