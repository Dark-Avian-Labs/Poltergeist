# Poltergeist

## Org standards

CI/README/validate conventions live in AppBase [`docs/org-standards/`](../AppBase/docs/org-standards/). This is a **Rust** app: quality gate is `scripts/validate` / `scripts/validate.ps1`. Release **Track B** (manual GitHub Release). PR jobs on `blacksmith-8vcpu-windows-2025`. It does **not** follow the AppBase design system.

Token syntax lives in `TUTORIAL.md`. Product packaging lives in `README.md`.

## Crates

`poltergeist-app` → `poltergeist-core` + `poltergeist-io` + `poltergeist-platform-win`. Core is pure domain (no FS/network). IO owns config, team share, DB lookups, and DeepL. Platform-win owns OS. App owns Slint, `base_dir`, edition detect, and inject orchestration.

All `unsafe` Win32 calls belong in `poltergeist-platform-win` `ffi.rs`. `hotkeys.rs` uses the `global-hotkey` crate, not `ffi`. Dual i18n: Slint `@tr` and Rust `i18n::tr` share the same `.po` files; switch both via `apply_bundled_translation`.

## Portable runtime

`base_dir()` is the exe directory, except when that directory is named `debug` or `release` under `target`: then it is the **workspace root**, so `cargo run` hits workspace `poltergeist.json`. Workspace `assets/` is compiled into the binary (Slint fonts/images, `include_bytes!` icons, winres exe icon). Release builds use `windows_subsystem = "windows"` (no console). Edition policy lives in the app, not `config.rs`. Resolve order: `--features admin-edition` (always Admin) → `POLTERGEIST_EDITION` → `_admin.flag` → User.

User edition syncs team share into the local tree/cache. Admin treats local `tree_team` as authoritative. Cargo always emits `poltergeist.exe`; release packaging **renames** the admin zip entry to `poltergeist-admin.exe`.

## Injection

UI label "Web Terminal" is core enum `TypingCompat`. Core `InjectionMode` and platform `injector::InjectionMode` are **separate** enums; the app maps them. Expand order before inject: includes → conditionals → DeepL (in **io**, not core) → platform inject. `{INCLUDE…}` max depth is 8. Team import is Admin-gated; team export is not.

DeepL uses raw reqwest. The workspace `deepl` crate is unused. PR also `cargo check -p poltergeist-app --features admin-edition --locked` after validate.
