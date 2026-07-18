---
type: Operations Guide
title: Portable runtime and editions
description: Next-to-exe config layout, base_dir resolution, and User vs Admin edition detection.
tags: [operations, config, editions, portable]
timestamp: 2026-07-19T01:10:00Z
---

# Portable runtime and editions

Poltergeist is designed to run without installer or registry dependency for its own data. Runtime files live beside the executable (with a special case under Cargo’s `target/` tree). Edition policy lives in the **app**, not in `poltergeist-io`. This guide configures the process that [hotkey to injection](../workflows/hotkey-to-injection.md) runs inside and is produced by [build and CI](build-and-ci.md).

## `base_dir()`

Defined in `crates/poltergeist-app/src/main.rs`:

- Normally: directory containing the executable
- If that directory is named `debug` or `release` and its parent is `target`: use the **workspace root** instead

So `cargo run` reads/writes workspace-root `poltergeist.json` and can see `assets/`, while a packaged zip keeps data next to the exe.

## Config files

Handled by `poltergeist_io::config`:

| File | Role |
|------|------|
| `poltergeist.json` | Primary config (`CONFIG_FILENAME`) |
| `poltergeist-defaults.json` | Optional bootstrap when primary missing |
| `team_cache/` | Cached team pack + databases (`CACHE_DIRNAME`) |
| `_admin.flag` | Beside base dir → Admin edition (when feature/env unset) |

`load` uses primary JSON, else defaults, then `contracts::merge_into_default`. `save` writes via `.json.tmp` then rename. `is_first_run` is `!poltergeist.json.exists()`.

## Edition resolution

`detect_edition(base_dir)` order:

1. Built with `--features admin-edition` → always **Admin**
2. Else `POLTERGEIST_EDITION=admin|user` (case-insensitive trim)
3. Else `_admin.flag` present beside base dir → Admin
4. Else **User**

User edition syncs team share into the local tree/cache; Admin treats local `tree_team` as authoritative while still probing share status. Single-instance mutex names differ (`Global\PoltergeistSnippetManager` vs `.Admin`).

## Where to start

- Path + edition: `crates/poltergeist-app/src/main.rs` (`base_dir`, `detect_edition`)
- Load/save: `crates/poltergeist-io/src/config.rs`
- Product notes: [README.md](../../README.md) — User/admin editions, config files

## Watch out for

- Do not document edition logic as living in `config.rs` — only path helpers live there.
- Feature-built admin binaries ignore env and `_admin.flag`.
- Nightly CI may rename the admin exe in the zip; that does not change Cargo’s output name. See [build and CI](build-and-ci.md).

## Key sources

- `crates/poltergeist-app/src/main.rs`
- `crates/poltergeist-io/src/config.rs`
- `crates/poltergeist-platform-win/src/single_instance.rs`
- [README.md](../../README.md)
