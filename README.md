# Poltergeist

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![CI](https://github.com/Dark-Avian-Labs/Poltergeist/actions/workflows/ci.yml/badge.svg)](https://github.com/Dark-Avian-Labs/Poltergeist/actions/workflows/ci.yml)
[![PR](https://github.com/Dark-Avian-Labs/Poltergeist/actions/workflows/pr.yml/badge.svg)](https://github.com/Dark-Avian-Labs/Poltergeist/actions/workflows/pr.yml)
![Rust 1.77+](https://img.shields.io/badge/rust-1.77%2B-B7410E?logo=rust&logoColor=white)
![Platform: Windows](https://img.shields.io/badge/platform-Windows_10%2F11-0078D6?logo=microsoft&logoColor=white)
![Slint](https://img.shields.io/badge/UI-Slint-41CD52?logo=slint&logoColor=white)
![i18n: EN · DE · ES · FR](https://img.shields.io/badge/i18n-EN%20·%20DE%20·%20ES%20·%20FR-6C7A89)
[![Cursor](https://img.shields.io/badge/Cursor-IDE-141414?logo=cursor&logoColor=white)](https://cursor.com)

A portable Windows snippet manager. Press a global hotkey, pick a snippet
from a nested popup at your mouse cursor, and watch it get typed or pasted
into whichever field had focus.

Built as a spiritual successor to GhostWriter and an alternative to PhraseExpress.

> **New here?** The full syntax reference (tokens, operators, filters, and
> worked examples) lives in **[TUTORIAL.md](./TUTORIAL.md)**.
> This README covers build/run, packaging, editions, team share modes, and troubleshooting.

## Features

- **Global hotkey** (default `Ctrl+Alt+Space`) opens a nested popup at the cursor.
- **Snippets and folders** with unlimited nesting. Drag-and-drop to reorder and re-nest.
- **Four injection modes** per snippet:
  - `clipboard (CTRL+V)` - backup / Ctrl+V / restore.
  - `clipboard (Shift+INS)` - same, using Shift+Insert for terminal surfaces.
  - `typing (Key Events)` - real key events.
  - `typing (Web Terminal)` - Win32 `SendInput` path using VK + scan codes for keycode-sensitive web terminals.
- **Rich token language** - dates, clipboard, waits, named keys, key combos, DeepL translation, context variables, database lookups, snippet includes, and `{IF}/{ELSIF}/{ELSE}/{END}` conditionals.
- **Team snippets over share or HTTP(S)** - Team tab can read from UNC/local folders or HTTP(S) endpoints; cache fallback is automatic when remote is unavailable.
- **Per-folder hotkeys** - assign a hotkey to any top-level folder for direct submenu entry.
- **Context-aware filtering** - regex capture groups become variables for snippet/folder `Show when...` rules.
- **CSV/XLSX lookups** - use `{DATABASE=...}` against team databases.
- **Portable runtime** - config and cache live next to the executable; no installer or registry dependency.
- **Localized UI** - English, German, Spanish, and French.

## Workspace layout

- `crates/poltergeist-app` - desktop UI app crate (package `poltergeist-app`, binary `poltergeist`).
- `crates/poltergeist-core` - token engine, models, match/filter logic.
- `crates/poltergeist-io` - config, team-pack sync, DeepL and database IO.
- `crates/poltergeist-platform-win` - Windows integrations (hotkeys, focus, injection, single-instance helpers).

## Running from source

From workspace root:

```powershell
cargo run -p poltergeist-app --bin poltergeist
```

Requirements:

- Windows 10/11
- Rust toolchain (`rust-version = 1.77`)
- Visual Studio Build Tools (C++ workload), if linker tools are missing

Contributor checks (run the quality gate before committing or opening a PR):

```powershell
./scripts/validate.ps1
```

```bash
scripts/validate
```

Both entrypoints run the same checks:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

## Building portable executables

User build:

```powershell
cargo build -p poltergeist-app --release
```

Output:

- `target/release/poltergeist.exe`

Fixed admin build:

```powershell
cargo build -p poltergeist-app --release --features admin-edition
```

Output binary is still `target/release/poltergeist.exe`, but the feature pins
the runtime edition to Admin.

## User and admin editions

For the default binary (`poltergeist.exe`), edition is resolved in this order:

1. `POLTERGEIST_EDITION=admin|user`
2. `_admin.flag` file beside the executable
3. Fallback: user edition

When built with `--features admin-edition`, runtime ignores env/flag and is always Admin.

## CI build artifacts

On push to `main`, CI builds and uploads two Windows zip artifacts to the
workflow run for debugging/verification only (CI does **not** publish a GitHub
Release):

- `poltergeist-user-windows.zip` (contains `poltergeist.exe`)
- `poltergeist-admin-windows.zip` (contains `poltergeist-admin.exe`)

When present, `assets/` is packaged alongside the executable.

## Releasing

Releases are published manually via the GitHub Actions **Poltergeist Release**
workflow:

1. Go to **Actions → Poltergeist Release → Run workflow**.
2. Optionally set `version` (e.g. `0.1.0`); leave empty to use `LATEST.VER`.
3. Optionally toggle `prerelease` / `draft`.

The workflow resolves the version (input or `LATEST.VER`), builds the user and
admin Windows editions, and publishes a GitHub Release tagged `vX.Y` (or
`vX.Y.Z`) with both zips attached.

## Scripts

| Script | Description |
| ------ | ----------- |
| `scripts/validate` / `scripts/validate.ps1` | Quality gate: `cargo fmt --check`, `cargo clippy -D warnings`, and `cargo test` (workspace, `--locked`). |

## Team share modes

`Options > Team share > Share path` supports:

- UNC/local folders (examples: `\\server\share\poltergeist`, `T:\Poltergeist`)
- HTTP(S) base URLs where these files are downloadable:
  - `{base}/manifest.json`
  - `{base}/team.poltergeist.json`
  - optional `{base}/databases/<name>` files listed in the manifest

Publishing from the app is supported for folder/UNC shares; HTTP(S) is read-only.

## Config and runtime files

Runtime data is portable and stored beside the executable:

- `poltergeist.json` - primary config
- `poltergeist-defaults.json` - optional bootstrap defaults
- `team_cache/` - cached team pack and database files

## DeepL and TLS

Network requests use `reqwest` with `rustls-tls-native-roots`, so the OS trust
store is included (useful for many corporate TLS interception setups).

## Tutorial

See **[TUTORIAL.md](./TUTORIAL.md)** for token syntax, conditionals,
filters, and full examples.

## Development

Agent-oriented docs: [openwiki/quickstart.md](openwiki/quickstart.md).
Org engineering standards: AppBase `docs/org-standards/`.

## License

Apache 2.0. See [LICENSE](LICENSE).