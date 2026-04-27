# Canon Reasoner

## Role
`canon_reasoner` reasons over canon constraints, forbidden edges, and authority laws.

## Owns
- `canon_reasoner_record_id`
- `scope_id`
- `state`
- `input_ref_set`
- `output_ref_set`

## Consumes
- `l6.3-snapshot-plane`
- `l6.4-index-plane`
- `l6.5-derived-plane`
- `l6.6-artifact-plane`
- `l6a.1-context-evidence-packs`
- `l6a.4-safety-gates`

## Emits
- cold planning/meta records
- reports or routing decisions
- bounded orchestration summaries

## Never owns
- hot mutation ownership
- editor widget state
- undeclared lower-layer truth
