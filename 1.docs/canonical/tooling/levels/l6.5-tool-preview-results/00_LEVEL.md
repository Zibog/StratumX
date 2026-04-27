# Tool Preview Results

## Role
`tool_preview_results` publishes preview result refs and status summaries returned by preview_runtime.

## Owns
- `preview_result_id`
- `preview_request_id`
- `result_ref`
- `result_state`
- `freshness_epoch`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_preview_results` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
