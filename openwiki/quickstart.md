---
type: Repository Overview
title: Quickstart
description: Entrypoint for agents and humans working on Poltergeist, the Windows snippet manager.
tags: [quickstart, overview]
timestamp: 2026-07-19T01:20:00Z
---

# Poltergeist OpenWiki

Poltergeist is a **portable Windows snippet manager**: a global hotkey opens a nested popup at the mouse cursor, then the selected snippet is expanded (tokens, conditionals, optional DeepL) and injected into the previously focused field. It targets GhostWriter / PhraseExpress-style workflows for individuals and teams.

Audience: contributors and agents changing the Rust/Slint desktop app, token engine, team sync, or Windows injection paths.

## Stack and layout

Cargo workspace (`rust-version = 1.77`, edition 2021) with four crates:

| Crate | Role |
|-------|------|
| `poltergeist-app` | Slint UI binary `poltergeist` |
| `poltergeist-core` | Models, match rules, token expand |
| `poltergeist-io` | Config, team pack, DB, DeepL HTTP |
| `poltergeist-platform-win` | Hotkeys, focus, injection, single-instance |

Human docs: [README.md](../README.md) (build/packaging), [TUTORIAL.md](../TUTORIAL.md) (token syntax).

## Run and develop

```powershell
cargo run -p poltergeist-app --bin poltergeist
```

Requirements: Windows 10/11, Rust 1.77+, Visual Studio Build Tools (C++ linker) if linking fails.

Contributor checks: `cargo fmt --all`, `cargo check --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.

Release user build: `cargo build -p poltergeist-app --release` → `target/release/poltergeist.exe`.
Admin-pinned build: same with `--features admin-edition`. Details in [build and CI](operations/build-and-ci.md) and [portable runtime](operations/portable-runtime-and-editions.md).

## Concept map

- [Workspace layout](architecture/workspace-layout.md) — crate boundaries and dependencies
- [Slint UI and i18n](architecture/slint-ui-and-i18n.md) — MainWindow, SnippetPopup, dual localization
- [Win32 FFI layer](architecture/win32-ffi.md) — centralized unsafe Win32 wrappers
- [Hotkey to injection](workflows/hotkey-to-injection.md) — main runtime path
- [Import / export picker](workflows/import-export-picker.md) — personal/team tree transfer UI
- [Models and tokens](domain/models-and-tokens.md) — config trees, expand pipeline, filters
- [Portable runtime and editions](operations/portable-runtime-and-editions.md) — paths, config files, User/Admin
- [Team share and DeepL](integrations/team-share-and-deepl.md) — UNC/HTTP packs, databases, translation
- [Build and CI](operations/build-and-ci.md) — local packages and GitHub Actions
- [Contributor checks](testing/contributor-checks.md) — tests and CI gates

## Agent gotchas

- **Windows-first.** Product behavior and CI runners are Windows; platform crate stubs elsewhere.
- **`base_dir()`** uses the exe directory, but when the exe lives under `target/debug` or `target/release` it redirects to the **workspace root** so `cargo run` finds `poltergeist.json` / assets.
- **Edition** is resolved in the app (`detect_edition`), not in `poltergeist-io`: feature `admin-edition` → env `POLTERGEIST_EDITION` → `_admin.flag` → User.
- **CI renames** the admin release binary to `poltergeist-admin.exe` in the zip; Cargo still emits `poltergeist.exe`.
- Domain type is `Node` / `PoltergeistConfig` (not `TreeNode` / `Config`). UI label “Web Terminal” maps to `InjectionMode::TypingCompat`.
- `picker.rs` is the **import/export** tree UI, not the snippet popup (`SnippetPopup` in Slint).
- Dual i18n: Slint `@tr` and Rust `i18n::tr` share the same `.po` files — switch both via `apply_bundled_translation`.
- All platform `unsafe` Win32 calls belong in `ffi.rs`; other modules stub off-Windows.
- DeepL talks to the **Free** API via raw `reqwest`; the `deepl` Cargo dependency is unused in source.
- Full token syntax lives in [TUTORIAL.md](../TUTORIAL.md) — do not duplicate it wholesale here.
