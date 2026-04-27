# Published tool-focus fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| focus_ref_id | FocusRefId | required | published focus binding identity | stable for the visible focus instance |
| focused_subject_ref | SubjectRef | required | currently focused subject reference | must resolve through declared refs |
| focus_source | FocusSource | required | source class for the focus assignment | must use declared enum |
| focus_scope | ScopeRef | required | scope where focus is valid | must resolve through declared refs |
| focus_epoch | Epoch | required | freshness marker for focus change | advances only on semantic focus change |
| focus_priority | FocusPriority | required | priority class for competing focus claims | must use declared enum |

## Publication law
This file freezes the externally visible publication contract for tool focus refs. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.
