# Tool Inspection Views

## Role
`tool_inspection_views` publishes structured inspection views generated from snapshots for inspectors and diagnostics.

## Owns
- `inspection_view_id`
- `target_ref`
- `field_set_ref`
- `view_generation_epoch`
- `view_kind`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_inspection_views` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
