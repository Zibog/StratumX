# tool_diagnostics_views internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| diagnostics_view_row_id | DiagnosticsViewRowId | required | canonical storage row identity | stable within the sidecar store |
| diagnostics_view_id | DiagnosticsViewId | required | foreign key to the published diagnostics view | must resolve to a root-level row |
| sort_profile | DiagnosticsSortProfile | required | sort policy used to assemble the view | must use declared enum or profile ref |
| selection_binding_ref | SelectionBindingRef | optional | binding to a selected diagnostics subject | present only when diagnostics and selection are coupled |
| retained_window | DiagnosticsWindowClass | required | time/window class retained by the view | must use declared enum |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_diagnostics_views`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
