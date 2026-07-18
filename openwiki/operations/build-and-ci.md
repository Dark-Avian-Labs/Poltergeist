---
type: Operations Guide
title: Build and CI
description: Local release builds, admin-edition packaging, and GitHub Actions on Blacksmith.
tags: [operations, build, ci, release]
timestamp: 2026-07-19T01:10:00Z
---

# Build and CI

How binaries are produced locally and on `main`. Edition semantics are described in [portable runtime and editions](portable-runtime-and-editions.md); quality gates overlap [contributor checks](../testing/contributor-checks.md).

## Local builds

From workspace root (Windows):

```powershell
cargo build -p poltergeist-app --release
# → target/release/poltergeist.exe

cargo build -p poltergeist-app --release --features admin-edition
# same output path; runtime pinned to Admin
```

`cargo run -p poltergeist-app --bin poltergeist` for development. Needs Rust 1.77+ and VS C++ build tools when the linker is missing.

When packaging manually, include `assets/` beside the exe if present (CI does this for nightlies).

## GitHub Actions

### PR — `.github/workflows/pr.yml`

- Trigger: `pull_request`
- Runner: `blacksmith-4vcpu-windows-2025`
- Steps: `cargo fmt --check` → `clippy -D warnings` → `cargo test --workspace`
- No artifacts or Discord

### Main — `.github/workflows/ci.yml`

| Job | Runner | Role |
|-----|--------|------|
| `pre-checks` | Windows Blacksmith | fmt, clippy, test, `cargo check -p poltergeist-app --features admin-edition` |
| `build` | Windows Blacksmith | Release user zip + admin feature build renamed to `poltergeist-admin.exe`; artifact `poltergeist-nightly-windows-zips` |
| `release` | Ubuntu Blacksmith | Force-move `nightly` tag; GitHub release with both zips |
| `discord-status` | Ubuntu | `always()` on main; webhook via secrets |

Triggers: push to `main` (path-filtered on Cargo/crates/assets/workflows) and `workflow_dispatch`. Checkout: `useblacksmith/checkout@v1`.

Published zip names (README):

- `poltergeist-nightly-user-windows.zip` → `poltergeist.exe`
- `poltergeist-nightly-admin-windows.zip` → `poltergeist-admin.exe`

## Where to start

- Local: [README.md](../../README.md) — Running / Building portable executables
- Workflows: `.github/workflows/pr.yml`, `.github/workflows/ci.yml`
- Feature flag: `crates/poltergeist-app/Cargo.toml` (`admin-edition`)

## Watch out for

- Cargo always emits `poltergeist.exe`; **CI renames** the admin artifact. Do not assume a different Cargo binary name.
- Admin feature build also embeds the admin icon via `build.rs` / `CARGO_FEATURE_ADMIN_EDITION`.
- Path filters on `ci.yml` mean docs-only pushes may not rebuild nightlies.

## Key sources

- `.github/workflows/{pr,ci}.yml`
- [README.md](../../README.md)
- `crates/poltergeist-app/{Cargo.toml,build.rs}`
