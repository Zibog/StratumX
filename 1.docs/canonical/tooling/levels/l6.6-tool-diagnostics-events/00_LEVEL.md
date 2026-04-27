# Tool Diagnostics Events

## Role
`tool_diagnostics_events` carries bounded diagnostics events emitted by validation/build/release/runtime services.

## Owns
- `diagnostic_event_id`
- `diagnostic_kind`
- `severity`
- `source_scope_id`
- `ordering_cursor`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_diagnostics_events` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
