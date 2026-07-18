---
okf_version: "0.1"
---

# OpenWiki

- [Quickstart](quickstart.md) — Repository entrypoint for humans and agents

# Architecture

- [Workspace layout](architecture/workspace-layout.md) — Cargo crates and dependency direction
- [Slint UI and i18n](architecture/slint-ui-and-i18n.md) — MainWindow, SnippetPopup, dual localization
- [Win32 FFI layer](architecture/win32-ffi.md) — Centralized unsafe Win32 wrappers

# Workflows

- [Hotkey to injection](workflows/hotkey-to-injection.md) — Global hotkey through expand to inject
- [Import / export picker](workflows/import-export-picker.md) — Personal/team tree transfer UI

# Domain

- [Models and tokens](domain/models-and-tokens.md) — Config trees, match rules, expand pipeline

# Operations

- [Portable runtime and editions](operations/portable-runtime-and-editions.md) — Paths, config files, User/Admin
- [Build and CI](operations/build-and-ci.md) — Local releases and GitHub Actions

# Integrations

- [Team share and DeepL](integrations/team-share-and-deepl.md) — UNC/HTTP packs, databases, translation

# Testing

- [Contributor checks](testing/contributor-checks.md) — fmt, clippy, tests, CI gates
