# tool_task_requests internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| tool_task_request_row_id | ToolTaskRequestRowId | required | canonical storage row identity | stable within the sidecar store |
| tool_task_request_id | ToolTaskRequestId | required | foreign key to the published task request | must resolve to a root-level row |
| routing_bucket | TaskRoutingBucket | required | routing bucket selected for dispatch | must use declared enum |
| batch_group_ref | BatchGroupRef | optional | batch group used to combine compatible task requests | absent when the request runs standalone |
| last_dispatch_state | TaskDispatchState | required | latest dispatch state for the request row | must use declared enum |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_task_requests`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
