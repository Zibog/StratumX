# Tool Selection

## Role
`tool_selection` publishes selection refs emitted by editor surfaces so lower tooling services can address the same objects without owning UI state.

## Owns
- `selection_ref_set_id`
- `selected_ref_set`
- `selection_source`
- `selection_epoch`
- `selection_scope`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_selection` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
