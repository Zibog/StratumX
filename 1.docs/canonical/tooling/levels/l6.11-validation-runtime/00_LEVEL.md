# Validation Runtime

## Role
`validation_runtime` runs schema, dependency, legality, and boundary validation jobs over tooling/editor-facing data.

## Owns
- `validation_run_id`
- `validation_scope`
- `rule_set_id`
- `issue_set_ref`
- `outcome_state`

## Consumes
- `l6.3-snapshot-plane`
- `l6.4-index-plane`
- `l6.7-stream-plane`
- `l6.9-budget-runtime`

## Emits
- validation issue sets
- validation status streams
- gate decisions for build/release/editor surfaces

## Never owns
- fix-up ownership
- editor form mutation
- artifact build ownership
