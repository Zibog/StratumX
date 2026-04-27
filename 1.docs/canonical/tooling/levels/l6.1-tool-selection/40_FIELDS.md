# Published tool-selection fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| selection_ref_set_id | SelectionRefSetId | required | published identity for the selection set | stable for the visible selection epoch |
| selected_ref_set | RefOrRefSet | required | selected refs exposed to consumers | must resolve through declared refs or sidecar rows |
| selection_source | SelectionSource | required | source class for the selection change | must use declared enum |
| selection_epoch | Epoch | required | freshness marker for semantic selection change | advances only on semantic change |
| selection_scope | ScopeRef | required | scope bound to the selection | must resolve through declared refs |
| selection_cardinality | SelectionCardinality | required | count of selected subjects | must equal the materialized ref count |
| selection_projection | SelectionProjectionClass | required | projection class used to assemble the selection | must use declared enum |

## Publication law
This file freezes the externally visible publication contract for tool selection sets. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.
