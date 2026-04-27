# Proposal Runtime

## Role
`proposal_runtime` materializes assistant proposals before they are lowered or rejected.

## Owns
- `proposal_runtime_id`
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
