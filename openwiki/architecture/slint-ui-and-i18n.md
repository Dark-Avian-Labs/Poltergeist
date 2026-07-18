---
type: Architecture Overview
title: Slint UI and i18n
description: MainWindow and SnippetPopup structure, build-time Slint compile, and dual Slint/Rust localization.
tags: [architecture, ui, slint, i18n]
timestamp: 2026-07-19T01:20:00Z
---

# Slint UI and i18n

Desktop UI lives in one Slint file plus a small Rust i18n helper. Runtime orchestration still sits in [hotkey to injection](../workflows/hotkey-to-injection.md); import/export overlays are covered in [import/export picker](../workflows/import-export-picker.md).

## UI structure (`ui/main.slint`)

| Export | Role |
|--------|------|
| `Theme` global | Light/dark derived colors, accents, spacing, font family names |
| `MainWindow` | Editor: Personal/Team trees, snippet editor, status, modal overlays |
| `SnippetPopup` | Separate always-on-top cursor menu (hotkey path) |

Shared structs include `TreeRowData`, `NodePickerRow`, `TokenChip`, `ColorSwatchRow`. Private widgets (icon buttons, tree rows, hotkey capture, themed edits) sit above `MainWindow`.

**MainWindow** (~1080×720) property groups agents touch most often:

- Trees / selection / drag-drop (`personal_tree_rows`, `team_tree_rows`, `selected_*`, `drag-*`, `drop-*`)
- Settings-facing fields (hotkey, date format, theme/language indices, DeepL, team, context patterns)
- Chrome: `is_admin_edition`, `is_light_theme`, `hotkeys_paused`, `status_text`, `save_state_kind`
- Overlay flags: `show_options_panel`, `show_about_panel`, `show_review_panel`, `show_picker_panel`, `show_confirm_panel`, `show_translation_picker`, `show_token_popup`, `show_color_popup`, hotkey popups, `help_visible`

Key callbacks include `request_save`, tree CRUD / `move_*` / `toggle_*_folder`, `apply_settings`, picker/confirm/review handlers, `build_and_insert_token`, `open_popup` / `close_popup`, DeepL validate, team publish/refresh, `format_hotkey`.

**SnippetPopup** props: `main-entries`, `sub-entries`, `submenu-visible`, `open-folder-row`, `is_light`. Callbacks: `main_row_clicked`, `folder_hover` / `folder_leave`, `sub_selected`, `dismissed` (Escape).

Both windows sync `Theme.is-light` from their light-theme properties. UI copy uses `@tr("...")`.

## Build (`build.rs`)

1. Compile `ui/main.slint` with style from `SLINT_STYLE` or default `fluent-dark`
2. If `lang/` exists → `with_bundled_translations(lang_dir)`; rerun on every `.po`
3. On Windows, embed `AppIconAdmin.ico` when `CARGO_FEATURE_ADMIN_EDITION`, else `AppIcon.ico` (via `winres`; failure warns and continues)

## Dual i18n

| Layer | Mechanism |
|-------|-----------|
| Slint `@tr` | Bundled gettext catalogs + `slint::select_bundled_translation` |
| Rust strings | `i18n::tr` / `tr_format` over the **same** `.po` files via `include_str!` |

Locales: English (source / empty locale), `de`, `es`, `fr` under `lang/{de,es,fr}/LC_MESSAGES/poltergeist-app.po`.

`apply_bundled_translation` in `main.rs` always updates **both** `i18n::set_locale` and `select_bundled_translation`. Called at startup from `settings.language` and again on Options save. Empty / `"en"` → Slint target `""`.

`tr_format` substitutes `{0}`, `{1}`, … after lookup (`{{` / `}}` escape). Unknown locale falls back to English source msgids.

Maintenance helpers under `lang/`: `_generate_po.py` (harvest from an external Python tree path), `_annotate_tr.py`, `_annotate_status_tr.py` — one-shot / check tools, not part of the normal build.

## Where to start

- UI: `crates/poltergeist-app/ui/main.slint`
- Build: `crates/poltergeist-app/build.rs`
- Rust i18n: `crates/poltergeist-app/src/i18n.rs`
- Locale switch: `apply_bundled_translation` in `crates/poltergeist-app/src/main.rs`

## Watch out for

- Changing language only works if both Slint and Rust catalogs stay in sync via `apply_bundled_translation`.
- New UI string → `@tr` **and** msgid/msgstr in all three `.po` files. New Rust status/dialog copy → `i18n::tr` / `tr_format` with the exact English msgid.
- Not every string is translated (e.g. some help text remains hardcoded English).
- Rust strings already pushed to the status bar do not auto-retranslate on locale change; subsequent `tr` calls do. Slint `@tr` nodes refresh via `select_bundled_translation`.
- `_generate_po.py` hardcodes an external Python path — regenerate may fail if that tree is missing.

## Key sources

- `crates/poltergeist-app/ui/main.slint`
- `crates/poltergeist-app/{build.rs,src/i18n.rs,src/main.rs}`
- `crates/poltergeist-app/lang/{de,es,fr}/LC_MESSAGES/poltergeist-app.po`
