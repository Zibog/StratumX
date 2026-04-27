# Preview Runtime

## Role
`preview_runtime` materializes disposable previews from declared requests without mutating authoritative truth.

## Owns
- `preview_run_id`
- `preview_request_id`
- `preview_kind`
- `source_snapshot_set`
- `result_ref`

## Consumes
- `l6.3-snapshot-plane`
- `l6.4-index-plane`
- `l6.5-derived-plane`
- `l6.7-stream-plane`
- `l6.9-budget-runtime`
- `l6.4-tool-preview-requests`
- `l6.5-tool-preview-results`

## Emits
- preview result refs
- preview progress streams
- cancel/supersede decisions under pressure

## Never owns
- authoritative mutation
- artifact permanence
- editor widget ownership
