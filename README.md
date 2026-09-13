<p align="center">
  <img src="https://raw.githubusercontent.com/Dark-Avian-Labs/.github/refs/heads/main/banner.png" alt="Dark Avian Labs">
</p>

# Poltergeist

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg?style=flat-square)](LICENSE)
[![PR](https://img.shields.io/github/actions/workflow/status/Dark-Avian-Labs/Poltergeist/pr.yml?style=flat-square&label=PR)](https://github.com/Dark-Avian-Labs/Poltergeist/actions/workflows/pr.yml)
![Rust 1.82+](https://img.shields.io/badge/rust-1.82%2B-B7410E?logo=rust&logoColor=white&style=flat-square)
![Platform: Windows](https://img.shields.io/badge/platform-Windows_10%2F11-0078D6?logo=microsoft&logoColor=white&style=flat-square)
![Slint](https://img.shields.io/badge/UI-Slint-41CD52?logo=slint&logoColor=white&style=flat-square)
![i18n: EN · DE · ES · FR](https://img.shields.io/badge/i18n-EN%20·%20DE%20·%20ES%20·%20FR-6C7A89?style=flat-square)
[![Cursor](https://img.shields.io/badge/Cursor-IDE-141414?logo=cursor&logoColor=white&style=flat-square)](https://cursor.com)

Poltergeist is the snippet manager I reach for when a form wants the same paragraph for the hundredth time. Hit the hotkey (default `Ctrl+Alt+Space`), pick from a nested menu at the cursor, and it types or pastes into whatever had focus.

Tokens, conditionals, team shares, DeepL. Portable: config sits beside the exe. This is the Rust successor to GhostWriter, and the one we actually ship.

Token syntax: [TUTORIAL.md](TUTORIAL.md).

## Gotchas

- Windows 10/11 plus a Rust toolchain (`rust-version = 1.82`). If the linker is missing, install Visual Studio Build Tools with the C++ workload.
- Config and cache live beside the executable (`poltergeist.json`, `team_cache/`). `cargo run` is the exception: when the exe sits in `target/debug` or `target/release`, the workspace root is used instead.
- Debug builds keep a console. Release builds are windowed.
- Edition: `--features admin-edition` pins Admin; otherwise `POLTERGEIST_EDITION` → `_admin.flag` beside the exe → User. Cargo always emits `poltergeist.exe`; the release zip renames the admin binary to `poltergeist-admin.exe`.
- Team share: UNC/local folders are read/write. HTTP(S) is read-only. Cache fallback kicks in when remote is down.
- DeepL uses rustls with the OS trust store.

## License

Apache 2.0. See [LICENSE](LICENSE).
