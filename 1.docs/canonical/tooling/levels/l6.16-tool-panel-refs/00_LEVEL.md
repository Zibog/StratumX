# Tool Panel Refs

## Role
`tool_panel_refs` publishes panel refs visible to tooling services while keeping actual layout and widget ownership in editor.

## Owns
- `panel_ref_id`
- `panel_kind`
- `attachment_scope`
- `hosting_surface`
- `panel_epoch`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_panel_refs` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
