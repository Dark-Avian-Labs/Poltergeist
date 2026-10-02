<p align="center">
  <img src="https://raw.githubusercontent.com/Dark-Avian-Labs/.github/refs/heads/main/banner.png" alt="Dark Avian Labs">
</p>

# Poltergeist

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg?style=flat-square)](LICENSE)
![Rust 1.82+](https://img.shields.io/badge/rust-1.82%2B-B7410E?logo=rust&logoColor=white&style=flat-square)
![Platform: Windows](https://img.shields.io/badge/platform-Windows_10%2F11-0078D6?logo=microsoft&logoColor=white&style=flat-square)
![Slint](https://img.shields.io/badge/UI-Slint-41CD52?logo=slint&logoColor=white&style=flat-square)
![i18n: EN · DE · ES · FR](https://img.shields.io/badge/i18n-EN%20·%20DE%20·%20ES%20·%20FR-6C7A89?style=flat-square)
[![Cursor](https://img.shields.io/badge/Cursor-IDE-141414?logo=cursor&logoColor=white&style=flat-square)](https://cursor.com)

Poltergeist is a Windows snippet manager. Hit the hotkey, pick a line from a nested menu at the cursor, and it types or pastes into whatever had focus.

It is for the forms, tickets, and chats that want the same paragraph again. The Rust rewrite of GhostWriter, and the one we ship.

Token syntax lives in [TUTORIAL.md](TUTORIAL.md).

## Features

**A picker at the cursor.** The default hotkey is `Ctrl+Alt+Space`. The menu opens where you are typing, including a path aimed at a picky terminal.

**Snippets that fill themselves in.** Dates, includes, if and else, CSV lookups, and regex against the text around the cursor. A DeepL step can translate a snippet when you have a key saved in the app.

**Three ways out.** Clipboard, Shift+Insert, or typed input. You pick the one the target field will accept.

**A team library.** A UNC or local folder is read and write. An HTTP share is read only. If the share is down, the last cache is what the picker uses.

**English, German, Spanish, and French.** The UI follows the language you set. User and admin editions exist for machines where a normal user cannot change the install.

## What you should know

This is a desktop program. Config and cache sit beside the exe, so you can move the folder and take the snippets with you. Nothing about it asks you to sign in.

A debug build keeps a console window. A release build does not. DeepL stays quiet until you paste an API key into the settings. The key is stored with the config, not in an environment file.

## Self-hosting

Windows 10 or 11, and Rust 1.82 or newer. There is no `.env`. If the linker is missing, install Visual Studio Build Tools with the C++ workload.

```
cargo build --release
```

Run the exe from the folder where you want `poltergeist.json` to live. `cargo run` is the exception. When the exe sits in `target/debug` or `target/release`, settings are read from the repo root instead. Cargo always emits `poltergeist.exe`. The release zip renames the admin build to `poltergeist-admin.exe`. The admin edition is `--features admin-edition`, otherwise `POLTERGEIST_EDITION`, otherwise an `_admin.flag` file beside the exe, otherwise the user edition.

## License

Apache 2.0. See [LICENSE](LICENSE).
