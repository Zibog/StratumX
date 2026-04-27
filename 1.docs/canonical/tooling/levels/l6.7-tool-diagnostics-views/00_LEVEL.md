# Tool Diagnostics Views

## Role
`tool_diagnostics_views` publishes filtered and grouped diagnostics views for consumers that need stable issue presentations.

## Owns
- `diagnostic_view_id`
- `issue_set_ref`
- `grouping_mode`
- `filter_digest`
- `freshness_epoch`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_diagnostics_views` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
