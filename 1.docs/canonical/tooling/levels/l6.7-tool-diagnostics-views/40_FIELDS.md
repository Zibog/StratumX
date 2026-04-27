# Published diagnostics-view fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| diagnostics_view_id | DiagnosticsViewId | required | published diagnostics view identity | stable for the active diagnostics surface |
| view_kind | DiagnosticsViewKind | required | kind of diagnostics projection | must use declared enum |
| event_set_ref | RefOrRefSet | required | event set displayed by the diagnostics view | must resolve through declared refs or view rows |
| filter_profile | DiagnosticsFilterProfile | optional | named filter profile for the view | must resolve or use declared enum when present |
| view_epoch | Epoch | required | freshness marker for the diagnostics projection | advances on semantic view change |
| owner_panel_ref | PanelRef | required | panel hosting the diagnostics view | must resolve through panel refs |

## Publication law
This file freezes the externally visible publication contract for diagnostics views. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.
