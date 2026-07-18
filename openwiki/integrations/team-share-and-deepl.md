---
type: Integration
title: Team share and DeepL
description: UNC/HTTP team packs, database lookups, cache fallback, and DeepL Free API translation.
tags: [integrations, team, deepl, database]
timestamp: 2026-07-19T01:10:00Z
---

# Team share and DeepL

External data paths used during expand and team UI. Cache and config paths depend on [portable runtime](../operations/portable-runtime-and-editions.md). Lookups participate in [models and tokens](../domain/models-and-tokens.md) via `DatabaseLookup` and `{TRANSLATION…}` blocks.

## Team pack (`team_pack.rs`)

Share root contents:

- `manifest.json`
- `team.poltergeist.json`
- optional `databases/<name>` files listed in the manifest

Local cache: `{base_dir}/team_cache/`.

| Share path | Behavior |
|------------|----------|
| Empty | Unconfigured / use cache if present |
| UNC or local folder | Read root; on success refresh local cache; else fall back to cache |
| `http://` / `https://` | GET manifest + tree; refresh HTTP cache (DBs from manifest); **publish not supported** |

Key APIs: `is_http_share`, `share_root`, `probe_status`, `read_pack_sync`, `publish_to_share`, `cache_dir`.

`ShareStatus`: `Unconfigured` | `Reachable` | `Cached` | `Unreachable`.

HTTP client: 45s timeout, 15s connect, User-Agent `Poltergeist/{CARGO_PKG_VERSION}`.

Publishing from the app works for folder/UNC shares only; HTTP(S) requires manual upload of the pack files.

Version fields to keep straight: config `PoltergeistConfig.version` (default 2); team manifest version (bumped on publish); written team tree file version is **1**.

## Databases (`database.rs`)

`DatabaseRegistry::load_from_sources(share_path, cache_path)` prefers `{share}/databases`, else `{cache}/databases`. Supports `.csv` and `.xlsx`/`.xlsm` (first sheet). Key column is first; names/columns lowercased; first duplicate key wins. Lookup without column returns `__key__`. Powers `{DATABASE=file,key,column}` during expand.

## DeepL (`translation.rs`)

Uses **raw reqwest** against Free endpoints:

- `https://api-free.deepl.com/v2/usage`
- `https://api-free.deepl.com/v2/translate`

API key from `settings.deepl_api_key`. Snippet blocks: `{TRANSLATION[=:][src>]tgt}…{TRANSLATION_END}` (body may span lines; inner tokens expand before translate). The workspace `deepl` crate dependency is **not** used by this module.

`TranslationService` APIs include `validate`, `translate_plain_text`, `expand_translations`, and helpers for review/prompt flows in the app.

## Where to start

- Team: `crates/poltergeist-io/src/team_pack.rs`
- DB: `crates/poltergeist-io/src/database.rs`
- DeepL: `crates/poltergeist-io/src/translation.rs`
- Product setup: [README.md](../../README.md) — Team share modes, DeepL and TLS
- Syntax: [TUTORIAL.md](../../TUTORIAL.md) — `{DATABASE}`, `{TRANSLATION}`

## Watch out for

- TLS uses `reqwest` with `rustls-tls-native-roots` (OS trust store) — relevant for corporate interception.
- HTTP share is read-only from the app; do not expect `publish_to_share` to succeed there.
- User edition polls the share on a timer; Admin keeps local team tree authoritative.

## Key sources

- `crates/poltergeist-io/src/{team_pack,database,translation}.rs`
- [README.md](../../README.md)
