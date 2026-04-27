# tool_selection internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| selection_row_id | SelectionRowId | required | canonical storage identity for the sidecar row | stable within the sidecar store |
| selection_ref_set_id | SelectionRefSetId | required | foreign key to the published selection set | must resolve to a root-level row |
| selection_diff_ref | SelectionDiffRef | optional | internal diff used to compute changes from the prior selection | must be absent for the first visible selection |
| selection_materialization_cursor | Cursor | required | cursor used for the current materialized projection | must match the visible epoch/cursor pair |
| selection_owner_session_id | ToolSessionId | required | owning tool session for the selection row | must resolve through tool-session publications |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_selection`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
