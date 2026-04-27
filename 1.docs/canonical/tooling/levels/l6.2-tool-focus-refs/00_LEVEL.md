# Tool Focus Refs

## Role
`tool_focus_refs` publishes current focus refs for one active inspection/interaction target without owning widget focus behavior.

## Owns
- `focus_ref_id`
- `focused_target_ref`
- `focus_owner_surface`
- `focus_epoch`
- `focus_mode`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_focus_refs` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
