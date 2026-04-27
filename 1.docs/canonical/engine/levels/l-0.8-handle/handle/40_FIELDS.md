# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| stable_handle | Stable public handle | Required | Legal at crate boundaries | engine_handle |
| dense_execution_handle | Dense traversal token | Optional | Internal to traversal plans only | engine_handle |
| generation | Generation counter | Required | For temporal validity tracking | engine_handle |
| validation_result | Validation outcome | Required | Generation-aware check result | engine_handle |
| invalidation_state | Invalidation status | Required | Tracks state transitions to non-authoritative | engine_handle |
| identity_reference | Identity domain reference | Required | From engine_identity | engine_handle |

## Invariant Rules

- Stable public handles are legal at crate boundaries
- Dense execution handles are internal traversal tokens only
- Dense execution handles may not escape compiled traversal plans or owning batches
- Validation is legal on boundary entry, diagnostics, and plan-build boundaries only
- Repeated stable-handle validation inside steady-state compiled traversal is illegal
- Generation-aware checks detect stale references after identity turnover
- Handle lifecycle must stay isolated from identity and storage to prevent temporal reference drift
