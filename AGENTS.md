# Poltergeist

## Org standards

Dark Avian Labs engineering conventions (README shape, CI/PR/release runners,
validate contract, OpenWiki, release tracks) live in AppBase
[`docs/org-standards/`](../AppBase/docs/org-standards/). Poltergeist follows
those docs: it is a **Rust** app, so the quality gate is `scripts/validate`
(bash) / `scripts/validate.ps1` (`cargo fmt --check`, `cargo clippy -D warnings`,
`cargo test`), and it uses release **Track B** (manual GitHub Release via
`.github/workflows/release.yml`). CI/PR jobs run on Blacksmith
`blacksmith-8vcpu-windows-2025` with `useblacksmith/checkout@v1`; Discord status
runs on `blacksmith-2vcpu-ubuntu-2404`.

## OpenWiki

This repository has documentation located in the /openwiki directory.

Start here:
- [OpenWiki quickstart](openwiki/quickstart.md)

OpenWiki includes repository overview, architecture notes, workflows, domain concepts, operations, integrations, testing guidance, and source maps.

When working in this repository, read the OpenWiki quickstart first, then follow its links to the relevant architecture, workflow, domain, operation, and testing notes.