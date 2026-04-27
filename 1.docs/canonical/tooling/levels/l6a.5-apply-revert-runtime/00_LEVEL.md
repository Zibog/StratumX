# Apply Revert Runtime

## Role
`apply_revert_runtime` owns assistant apply/revert records for accepted operations.

## Owns
- `apply_revert_runtime_id`
- `source_session_id`
- `state`
- `source_digest`
- `result_ref`

## Consumes
- `l6a.0-assistant-sessions`
- `l6.9-budget-runtime`
- `l6.7-stream-plane`

## Emits
- assistant runtime records
- bounded status streams
- lowering/apply/revert handoffs where legal

## Never owns
- direct authority mutation
- editor widget ownership
- undeclared hidden memory stores
