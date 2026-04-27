# Tool Task Results

## Role
`tool_task_results` publishes structured results of long-running tasks without owning their source truth.

## Owns
- `task_result_id`
- `task_request_id`
- `result_state`
- `result_ref`
- `finished_at_cursor`

## Consumes
- `l6.0-authority-core`
- `l6.0-tool-session`

## Emits
- published `tool_task_results` rows for downstream services
- bounded status or routing updates where applicable

## Never owns
- editor widget/layout ownership
- authority mutation truth not named by this sidecar
- hidden caches or silent side channels
