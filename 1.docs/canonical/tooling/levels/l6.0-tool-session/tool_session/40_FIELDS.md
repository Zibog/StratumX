# tool_session internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| tool_session_row_id | ToolSessionRowId | required | canonical storage row identity inside the sidecar | stable within the sidecar store |
| tool_session_id | ToolSessionId | required | foreign key to the published session identity | must resolve to a root-level publication row |
| activation_scope_snapshot | ScopeSnapshotRef | required | captured scope used to restore the session context | must match the scope visible at open time |
| last_visibility_cursor | Cursor | required | latest cursor observed by the session runtime | must advance monotonically |
| drain_reason | SessionDrainReason | optional | explicit internal reason for entering draining/closed state | required before terminal closure when closure is abnormal |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_session`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
