---
type: Workflow
title: Hotkey to injection
description: End-to-end path from global hotkey press through snippet expansion to text injection.
tags: [workflow, hotkey, injection, popup]
timestamp: 2026-07-19T01:10:00Z
---

# Hotkey to injection

This is the primary runtime workflow. It depends on [workspace layout](../architecture/workspace-layout.md), expands text via [models and tokens](../domain/models-and-tokens.md), and may call [team share and DeepL](../integrations/team-share-and-deepl.md) before platform inject.

## Flow

1. **Startup** (`main.rs`): logging → `base_dir()` → `detect_edition` → single-instance mutex → load config / team pack / databases → create Slint `MainWindow` + `SnippetPopup` → install `HotkeyManager` → 125ms poll timer (hotkeys, tray, outside-click dismiss). User edition also polls team share every 5 minutes.
2. **Hotkey event**: poll hides an already-visible popup, otherwise captures `current_foreground()` into `target_hwnd`, builds context from clipboard + `settings.context_patterns`, filters nodes by `Show when…` match rules, and opens `SnippetPopup` at the cursor (`open_popup_for_nodes`). Bindings cover the main hotkey plus per-folder shortcuts (`personal:{id}` / `team:{id}`).
3. **Selection**: user picks a snippet from the main list or hover submenu.
4. **`inject_snippet_now`**: resolve injection mode (snippet override or `settings.default_injection`) → `expand_includes` → `expand_conditionals` → optional DeepL review if `prompt_untranslated_before_paste` → DeepL expand when needed → `platform inject` with restored focus → clear `target_hwnd`.

Default inject timings in the app: paste delay ~60ms, clipboard restore ~250ms (`InjectParams`).

## Injection modes

| User / README label | Core / typical mapping |
|---------------------|-------------------------|
| clipboard (CTRL+V) | `Clipboard` |
| clipboard (Shift+INS) | `ClipboardShiftInsert` |
| typing (Key Events) | `Typing` |
| typing (Web Terminal) | `TypingCompat` (VK + scan codes) |

Implemented in `crates/poltergeist-platform-win/src/injector.rs` (clipboard backup/restore vs `SendInput` paths).

## Where to start

- Orchestration: `crates/poltergeist-app/src/main.rs` (`open_popup_for_nodes`, `inject_snippet_now`)
- Hotkeys: `crates/poltergeist-platform-win/src/hotkeys.rs`
- Focus / cursor: `focus.rs`, `cursor.rs`
- Popup UI: `SnippetPopup` in `crates/poltergeist-app/ui/main.slint` — see [Slint UI and i18n](../architecture/slint-ui-and-i18n.md)
- Injection OS calls: [Win32 FFI layer](../architecture/win32-ffi.md)

## Watch out for

- Do not confuse `picker.rs` (import/export checkbox tree) with the snippet popup — see [import/export picker](import-export-picker.md).
- Folder hotkeys and main hotkey share the same poll loop; toggling the popup closed is intentional on re-press.
- Admin vs User changes which team tree source is authoritative (local admin tree vs synced share) — see [portable runtime](../operations/portable-runtime-and-editions.md).

## Key sources

- `crates/poltergeist-app/src/main.rs`
- `crates/poltergeist-platform-win/src/{hotkeys,focus,injector,cursor}.rs`
- [TUTORIAL.md](../../TUTORIAL.md) — injection mode guidance for authors
