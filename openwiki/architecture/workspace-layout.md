---
type: Architecture Overview
title: Workspace layout
description: Cargo workspace crates, dependency direction, and where runtime concerns live.
tags: [architecture, crates, monorepo]
timestamp: 2026-07-19T01:10:00Z
---

# Workspace layout

Poltergeist is a single Cargo workspace (`Cargo.toml` members) with a clear layering: **core** (pure domain) ← **io** / **platform** (side effects) ← **app** (UI + orchestration).

## Crates

| Package | Path | Responsibility |
|---------|------|----------------|
| `poltergeist-core` | `crates/poltergeist-core` | `models`, `tokens`, `context`, `contracts`; no filesystem or network |
| `poltergeist-io` | `crates/poltergeist-io` | `config`, `team_pack`, `database`, `translation` |
| `poltergeist-platform-win` | `crates/poltergeist-platform-win` | `hotkeys`, `focus`, `injector`, `cursor`, `single_instance`, `theme`; Windows `ffi` |
| `poltergeist-app` | `crates/poltergeist-app` | Binary `poltergeist`; Slint UI; wires hotkey → expand → inject |

`poltergeist-core` exports `APP_NAME = "Poltergeist"`. Shared workspace deps (serde, tokio, slint, windows, reqwest, …) are declared in the root `Cargo.toml`.

## Dependency direction

```text
poltergeist-app
  ├── poltergeist-core
  ├── poltergeist-io        (uses core models / DatabaseLookup)
  └── poltergeist-platform-win
```

Keep domain logic and expand rules in **core**. Put portable file/HTTP I/O in **io**. Put OS integration in **platform-win**. Put Slint state machines and edition/path policy in **app** (`base_dir`, `detect_edition`, `inject_snippet_now`).

This layout depends on [portable runtime and editions](../operations/portable-runtime-and-editions.md) for where config lives, and dispatches user actions through [hotkey to injection](../workflows/hotkey-to-injection.md).

## Where to start

- App entry / orchestration: `crates/poltergeist-app/src/main.rs`
- Domain types: `crates/poltergeist-core/src/models.rs`
- Token expand: `crates/poltergeist-core/src/tokens.rs`
- Config + team + DeepL: `crates/poltergeist-io/src/{config,team_pack,translation}.rs`
- Injection: `crates/poltergeist-platform-win/src/injector.rs`
- UI: `crates/poltergeist-app/ui/main.slint` (compile via `build.rs`) — see [Slint UI and i18n](slint-ui-and-i18n.md)
- Win32 wrappers: [Win32 FFI layer](win32-ffi.md)

## Watch out for

- Platform crate compiles on non-Windows with stubs; real behavior is Windows-only. Keep new `unsafe` in `ffi.rs` only.
- Core `InjectionMode` and platform `injector::InjectionMode` are separate enums — the app maps between them when calling `inject`.
- Team publish, HTTP download, and DeepL validation belong in **io**, not the UI crate.

## Key sources

- Root `Cargo.toml` — members and workspace deps
- `crates/*/src/lib.rs` — public module lists
- `crates/poltergeist-app/Cargo.toml` — `admin-edition` feature, binary name
