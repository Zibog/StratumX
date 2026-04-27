# Tool Session

## Role
`tool_session` tracks public tool-session identity, lifecycle, and caller scope used by all sidecar traffic.

## Owns
- `tool_session_id`
- `session_origin`
- `attachment_scope`
- `session_state`
- `opened_at_cursor`

## Consumes
- `l6.0-authority-core`

## Emits
- published `tool_session` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
