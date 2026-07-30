---
type: Operations Guide
title: Build and CI
description: Local release builds, admin-edition packaging, and GitHub Actions on Blacksmith.
tags: [operations, build, ci, release]
timestamp: 2026-07-29T08:20:00Z
---

# Build and CI

How binaries are produced locally and on GitHub Actions. Edition semantics are described in [portable runtime and editions](portable-runtime-and-editions.md); quality gates overlap [contributor checks](../testing/contributor-checks.md).

## Local builds

From workspace root (Windows):

```powershell
cargo build -p poltergeist-app --release
# → target/release/poltergeist.exe

cargo build -p poltergeist-app --release --features admin-edition
# same output path; runtime pinned to Admin
```

`cargo run -p poltergeist-app --bin poltergeist` for development. Needs Rust 1.82+ and VS C++ build tools when the linker is missing.

When packaging manually, include `assets/` beside the exe if present (the release workflow does this).

## GitHub Actions

### PR — `.github/workflows/pr.yml`

- Trigger: `pull_request`
- Runner: `blacksmith-8vcpu-windows-2025`
- Steps: `scripts/validate.ps1` (fmt, clippy, test) then `cargo check -p poltergeist-app --features admin-edition --locked`
- No Actions artifacts or Discord

There is no push-to-`main` CI workflow; merge gates live on PRs only.

### Release — `.github/workflows/release.yml`

Manual `workflow_dispatch`. Builds and publishes on the same Windows job (no Actions artifact handoff).

| Job                 | Runner             | Role                                                                 |
| ------------------- | ------------------ | -------------------------------------------------------------------- |
| `prepare`           | Ubuntu Blacksmith  | Resolve version from input or `LATEST.VER`                           |
| `build-and-release` | Windows Blacksmith | Release user + admin zips; publish GitHub Release with both attached |
| `discord-status`    | Ubuntu             | `always()`; webhook via secrets                                      |

Published zip names:

- `poltergeist-<version>-user-windows.zip` → `poltergeist.exe`
- `poltergeist-<version>-admin-windows.zip` → `poltergeist-admin.exe`

## Where to start

- Local: [README.md](../../README.md) — Running / Building portable executables
- Workflows: `.github/workflows/pr.yml`, `.github/workflows/release.yml`
- Feature flag: `crates/poltergeist-app/Cargo.toml` (`admin-edition`)

## Watch out for

- Cargo always emits `poltergeist.exe`; **release packaging renames** the admin zip entry to `poltergeist-admin.exe`. Do not assume a different Cargo binary name.
- Admin feature build also embeds the admin icon via `build.rs` / `CARGO_FEATURE_ADMIN_EDITION`.

## Key sources

- `.github/workflows/{pr,release}.yml`
- [README.md](../../README.md)
- `crates/poltergeist-app/{Cargo.toml,build.rs}`
