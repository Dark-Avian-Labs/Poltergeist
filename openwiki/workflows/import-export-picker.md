---
type: Workflow
title: Import export picker
description: Checkbox tree UI for importing and exporting personal or team snippet trees.
tags: [workflow, import, export, picker]
timestamp: 2026-07-19T01:20:00Z
---

# Import / export picker

Secondary workflow for transferring snippet trees as JSON. This is **not** the hotkey [SnippetPopup](../architecture/slint-ui-and-i18n.md); logic lives in `picker.rs` and the Slint `show_picker_panel` overlay. Trees are `[Node]` graphs from [models and tokens](../domain/models-and-tokens.md).

## Purpose

`PickerPurpose`: `ExportPersonal` | `ExportTeam` | `ImportPersonal` | `ImportTeam`.

Typical path:

1. File dialog chooses path (before the panel opens)
2. `show_picker` builds a `PickerSession`, sets title/subtitle/OK label, flattens visible rows into Slint
3. User checks/expands nodes; Accept runs export write **or** stages import for merge/replace confirm

## Tree model (`picker.rs`)

- `PickerNode::from_node`: folders start **Checked + expanded**; snippets **Checked**
- `flatten`: DFS of visible rows; children walked only when folder expanded; builds parallel `visible_paths: Vec<Vec<usize>>`
- `toggle_check`: Checked ↔ Unchecked; Partial → Checked; folders recurse then `recompute_roll_up`
- `toggle_expand`: folders only
- Helpers: `set_all`, `count_checked`, `can_accept`, `format_summary`, `build_filtered`

`build_filtered` keeps checked snippets; drops unchecked folders; recurses Partial folders; rebuilds kept `Folder` metadata from the source node.

## Wiring (`main.rs`)

| Entry callback | Purpose | Source tree | Gate |
|----------------|---------|-------------|------|
| `on_import_personal` | `ImportPersonal` | parsed import file | none |
| `on_export_personal` | `ExportPersonal` | `tree_personal` | non-empty |
| `on_import_team` | `ImportTeam` | parsed import file | **Admin only** |
| `on_export_team` | `ExportTeam` | `team_tree` | non-empty (not Admin-gated) |

UI callbacks: `on_picker_toggle_check` / `on_picker_toggle_expand` (row index → `visible_paths[idx]`), select-all / clear-all, cancel, accept.

**Accept:**

- **Export\*** → `build_filtered` → pretty JSON `{"version":1,"tree":…}` → write `file_path` → clear session
- **Import\*** → stash `pending_filtered` → confirm dialog (`kind: "import_apply"`): Yes = **merge** (extend), No = **replace**, Cancel = abort

Import confirm regenerates IDs (`regenerate_ids`) and reinstalls hotkeys after applying the tree.

## Where to start

- Logic: `crates/poltergeist-app/src/picker.rs`
- Session / callbacks: `show_picker`, `refresh_picker_view`, `on_picker_*`, confirm handler in `crates/poltergeist-app/src/main.rs`
- Overlay: `show_picker_panel` in `crates/poltergeist-app/ui/main.slint`

## Watch out for

- Row index ≠ tree path — always resolve through `visible_paths` after flatten; collapsing changes indices.
- New sessions default to **everything checked**.
- Import is two-step: Accept only stages; merge/replace happens on confirm.
- Team **import** is Admin-gated; team **export** is not.
- Do not confuse this with `SnippetPopup` or token/color popups.

## Key sources

- `crates/poltergeist-app/src/picker.rs`
- `crates/poltergeist-app/src/main.rs` (picker + import confirm)
- `crates/poltergeist-app/ui/main.slint` (`show_picker_panel`)
