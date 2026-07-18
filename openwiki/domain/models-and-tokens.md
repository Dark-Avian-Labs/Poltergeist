---
type: Domain Concept
title: Models and tokens
description: Config trees, match rules, context extraction, and the snippet token expand pipeline.
tags: [domain, tokens, models, match]
timestamp: 2026-07-19T01:10:00Z
---

# Models and tokens

Domain types and expansion live in `poltergeist-core`. User-facing syntax and examples are canonical in [TUTORIAL.md](../../TUTORIAL.md). This page maps code, not a second tutorial. It is consumed by [hotkey to injection](../workflows/hotkey-to-injection.md) and fed by context / DB / DeepL integrations.

## Config shape

`PoltergeistConfig` (`models.rs`): `version` (default **2**), `settings`, `tree_personal`, `tree_team`.

`Node` is a serde-tagged enum: `Folder` | `Snippet`. Folders nest `children: Vec<Node>` and may carry `shortcut` + `match`. Snippets carry `text`, optional `injection`, `prompt_untranslated_before_paste`, `color`, `match`.

`Settings` includes hotkey, `default_injection`, date format, theme, DeepL key, language, `team_share_path`, `team_shortcuts`, `context_patterns`, window size, etc.

`InjectionMode` (serde `snake_case`): `Clipboard` (default), `ClipboardShiftInsert`, `Typing`, `TypingCompat`.

`contracts::merge_into_default` merges partial JSON into defaults and maps legacy `"tree"` → `tree_personal`.

## Context and visibility

`context::parse(text, patterns)` runs the first matching regex; named captures plus `_full` / `_raw` become variables for `{VAR=…}`, `{DATABASE=…}`, `{IF …}`, and “Show when…” filters.

Match rules (`MatchRule` / `MatchCondition` / `MatchOp`) back both conditionals and filters. Empty filter → always visible; `hide` / `never` / `no` → never in popup (helpers for `{INCLUDE=…}`). Expression helpers: `match_rule_from_expr` / `match_rule_to_expr`.

## Expand pipeline (`tokens.rs`)

Order matters:

1. `expand_includes` — `{INCLUDE=name}` / `{INCLUDE:name}`, max depth **8**
2. `expand_conditionals` — `{IF …}` / `{ELSIF|ELIF|ELSEIF}` / `{ELSE}` / `{END}`
3. Scalar / control token replace — `DATE`, `CLIPBOARD`, `VAR`, `DATABASE`, `WAIT`, keys/hotkeys; `{{` / `}}` → literal braces

Public expand helpers:

- `expand_for_clipboard` → plain `String` (strips wait/key segments)
- `expand_for_clipboard_segments` / `expand_for_typing` → `Segment` lists (`Text` | `Wait` | `Key` | `Hotkey`)

`DatabaseLookup` trait is implemented by io’s `DatabaseRegistry`. Argument tokens generally accept both `=` and `:` separators (see tutorial).

DeepL block expansion (`{TRANSLATION…}…{TRANSLATION_END}`) is handled in **io** after includes/conditionals, not inside core tokens.

## Where to start

- Types: `crates/poltergeist-core/src/models.rs`
- Expand: `crates/poltergeist-core/src/tokens.rs`
- Context: `crates/poltergeist-core/src/context.rs`
- Defaults merge: `crates/poltergeist-core/src/contracts.rs`
- Syntax reference: [TUTORIAL.md](../../TUTORIAL.md)

## Watch out for

- Naming: `Node` / `PoltergeistConfig`, not `TreeNode` / `Config`.
- `TypingCompat` is the “Web Terminal” mode in docs/UI.
- Include depth is hard-capped at 8; recursive helpers must stay under that.
- Core unit tests cover conditionals, includes, typing newlines, DB+$var, and context parse — extend those when changing expand semantics.

## Key sources

- `crates/poltergeist-core/src/{models,tokens,context,contracts}.rs`
- Root sample `poltergeist.json` (local runtime config shape)
