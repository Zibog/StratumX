# tool_preview_requests internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| preview_request_row_id | PreviewRequestRowId | required | canonical storage row identity | stable within the sidecar store |
| preview_request_id | PreviewRequestId | required | foreign key to the published request | must resolve to a root-level row |
| request_budget_ticket | BudgetTicketRef | optional | budget reservation used to service the preview | required when preview work is budget-gated |
| request_coalescing_group | PreviewCoalescingGroup | optional | group id used to merge redundant preview requests | must be absent when no coalescing occurs |
| last_dispatch_cursor | Cursor | optional | latest dispatch cursor emitted for this request | monotonic when present |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_preview_requests`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
