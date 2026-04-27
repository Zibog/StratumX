# Assistant Sessions

## Role
`assistant_sessions` tracks assistant conversation/session identity and ownership boundaries for one attached editor/tool context.

## Owns
- `assistant_session_id`
- `source_session_id`
- `state`
- `source_digest`
- `result_ref`

## Consumes
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
