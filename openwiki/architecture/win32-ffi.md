---
type: Architecture Overview
title: Win32 FFI layer
description: Centralized unsafe Win32 wrappers and how platform modules call them (with non-Windows stubs).
tags: [architecture, windows, ffi, platform]
timestamp: 2026-07-19T01:20:00Z
---

# Win32 FFI layer

All `unsafe` Win32 calls in `poltergeist-platform-win` are confined to `ffi.rs` (`#[cfg(windows)]` module). Higher-level modules own safe APIs and provide non-Windows stubs. This layer backs [hotkey to injection](../workflows/hotkey-to-injection.md) and [workspace layout](workspace-layout.md).

## Wrappers by concern

| Concern | `ffi` functions | Win32 |
|---------|-----------------|-------|
| Cursor / mouse | `get_cursor_pos`, `primary_mouse_buttons_down` | `GetCursorPos`, `GetAsyncKeyState` |
| Foreground | `get_foreground_window_hwnd`, `set_foreground_window` | `GetForegroundWindow`, `SetForegroundWindow` |
| Keyboard / SendInput | `send_keyboard_input`, `caps_lock_toggled_on`, `vk_key_scan_w`, `map_virtual_key_vk_to_vsc` | `SendInput`, `GetKeyState`, `VkKeyScanW`, `MapVirtualKeyW` |
| Theme registry | `query_apps_use_light_theme` | `AppsUseLightTheme` via `RegOpenKeyExW` / `RegQueryValueExW` |
| Single-instance | `create_global_mutex`, `close_handle_best_effort` | `CreateMutexW`, `CloseHandle` |
| Message box | `message_box_information` | `MessageBoxW` |

## Call sites

| Module | Uses |
|--------|------|
| `cursor.rs` | cursor position, primary buttons down |
| `focus.rs` | get/set foreground HWND |
| `injector.rs` | SendInput paths, Caps Lock, VkKeyScan; `map_virtual_key_vk_to_vsc` result currently discarded |
| `single_instance.rs` | global mutex acquire/drop; already-running dialog |
| `theme.rs` | system light-theme query |
| `hotkeys.rs` | **no** `ffi` — uses the `global-hotkey` crate |

## Stub behavior (`not(windows)`)

| Module | Stub |
|--------|------|
| `cursor` | `None` / `false` |
| `focus` | `None` / `false` (`WindowHandle` type differs: `isize` vs `i64`) |
| `theme` | `None` |
| `single_instance` | always `Acquired`; dialog no-op |
| `injector` | send helpers return `Ok(())` without injecting |

## Where to start

- Wrappers: `crates/poltergeist-platform-win/src/ffi.rs`
- Module root: `crates/poltergeist-platform-win/src/lib.rs`
- Consumers: `cursor.rs`, `focus.rs`, `injector.rs`, `single_instance.rs`, `theme.rs`

## Watch out for

- Do **not** add new `unsafe` Win32 calls outside `ffi.rs`; extend the wrapper table and keep stubs at the public modules.
- Mutex names differ by edition: `Global\PoltergeistSnippetManager` vs `.Admin`.
- If mutex create fails, `single_instance` may still treat the process as acquired (soft-disable) — see `CreateFailed` handling.
- Product and CI are Windows-first; stubs exist so the crate compiles elsewhere, not for full cross-platform behavior.

## Key sources

- `crates/poltergeist-platform-win/src/ffi.rs`
- `crates/poltergeist-platform-win/src/{cursor,focus,injector,single_instance,theme}.rs`
