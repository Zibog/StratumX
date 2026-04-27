# Context Evidence Packs

## Role
`context_evidence_packs` collects bounded evidence packs that assistant reasoning may read but not mutate.

## Owns
- `context_evidence_pack_id`
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
