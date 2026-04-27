# Tool View Refs

## Role
`tool_view_refs` publishes view refs for view-host coordination without taking ownership of rendered editor views.

## Owns
- `view_ref_id`
- `view_kind`
- `target_scope`
- `hosting_surface`
- `view_epoch`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_view_refs` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
