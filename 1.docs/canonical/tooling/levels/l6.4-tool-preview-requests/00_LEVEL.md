# Tool Preview Requests

## Role
`tool_preview_requests` carries preview requests from editor/tools into preview_runtime.

## Owns
- `preview_request_id`
- `request_kind`
- `target_ref_set`
- `quality_hint`
- `cancel_token`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_preview_requests` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
