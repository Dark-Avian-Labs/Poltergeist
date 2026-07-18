---
type: Testing Guide
title: Contributor checks
description: Local validation commands, existing unit tests, and CI quality gates.
tags: [testing, clippy, ci]
timestamp: 2026-07-19T01:10:00Z
---

# Contributor checks

How to validate changes before merge. CI details live in [build and CI](../operations/build-and-ci.md); domain expand behavior under test is described in [models and tokens](../domain/models-and-tokens.md).

## Local commands

From workspace root:

```powershell
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Also verify admin compile when touching edition/UI branding:

```powershell
cargo check -p poltergeist-app --features admin-edition
```

PR CI runs fmt check, clippy (`-D warnings`), and workspace tests on Windows Blacksmith. Main CI adds the admin-edition check and release packaging.

## What is tested today

**`poltergeist-core`** — inline `#[cfg(test)]` modules:

| Area | Coverage (high level) |
|------|------------------------|
| `tokens.rs` | IF/ELSIF/ELSE branches; INCLUDE + VAR; typing newline → enter; DATABASE with `$context` |
| `context.rs` | First-matching pattern, named groups, `_raw` |

**`poltergeist-io`**, **`poltergeist-platform-win`**, and **`poltergeist-app`** currently have little or no automated unit coverage — prefer manual Windows smoke (hotkey → popup → inject) for those layers, and extend core tests when changing expand/match semantics.

## Where to start

- Token/context tests: `crates/poltergeist-core/src/{tokens,context}.rs`
- CI gates: `.github/workflows/pr.yml`, `pre-checks` in `ci.yml`
- Manual scenarios: [TUTORIAL.md](../../TUTORIAL.md) worked examples + injection mode notes

## Watch out for

- Clippy is treated as errors (`-D warnings`) in CI — local clippy should match.
- Platform and injection bugs often need a real Windows session; stubs hide failures on other OSes.
- Changing token grammar without updating core tests is an easy regression.

## Key sources

- `crates/poltergeist-core/src/tokens.rs` / `context.rs` (test modules)
- `.github/workflows/pr.yml`
- [README.md](../../README.md) — Contributor checks
