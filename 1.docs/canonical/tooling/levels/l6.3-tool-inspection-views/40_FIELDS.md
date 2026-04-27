# Published inspection-view fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| inspection_view_id | InspectionViewId | required | published inspection view identity | stable for the visible inspection surface |
| inspected_subject_ref | SubjectRef | required | subject currently being inspected | must resolve through declared refs |
| inspection_projection | InspectionProjectionClass | required | projection class for the inspection view | must use declared enum |
| view_epoch | Epoch | required | freshness marker for inspection content | advances on semantic inspection change |
| validation_overlay_ref | ValidationOverlayRef | optional | associated validation overlay for the view | must resolve when present |
| diff_overlay_ref | DiffOverlayRef | optional | associated diff overlay for the view | must resolve when present |

## Publication law
This file freezes the externally visible publication contract for inspection views. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.
