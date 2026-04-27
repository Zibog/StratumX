# Workspace Runtime

## Role
`workspace_runtime` coordinates published workspace refs for selection, focus, panel, and view attachment without owning editor-local UI state.

## Owns
- `workspace_session_id`
- `published_selection_ref_set`
- `published_focus_ref_set`
- `published_panel_ref_set`
- `published_view_ref_set`

## Consumes
- `l6.0-authority-core`
- `l6.3-snapshot-plane`
- `l6.7-stream-plane`
- `l6.9-budget-runtime`
- `l6.1-tool-selection`
- `l6.2-tool-focus-refs`
- `l6.16-tool-panel-refs`
- `l6.17-tool-view-refs`

## Emits
- public coordination refs
- runtime-attach publications
- selection/focus/panel/view ref snapshots

## Never owns
- editor layout ownership
- widget tree ownership
- inspector form state or viewport camera state
