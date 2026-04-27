# Tool Task Requests

## Role
`tool_task_requests` carries long-running work requests that may end in build, preview, validation, or assistant tasks.

## Owns
- `task_request_id`
- `task_kind`
- `target_scope`
- `priority`
- `issuer_session_id`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_task_requests` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
