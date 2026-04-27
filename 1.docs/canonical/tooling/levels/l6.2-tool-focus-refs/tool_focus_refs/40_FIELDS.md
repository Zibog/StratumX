# tool_focus_refs internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| focus_row_id | FocusRowId | required | canonical storage row identity | stable within the sidecar store |
| focus_ref_id | FocusRefId | required | foreign key to the public focus binding | must resolve to a root-level row |
| prior_focus_ref | SubjectRef | optional | previous focused subject used for transition bookkeeping | absent on first acquisition |
| focus_transition_cursor | Cursor | required | cursor at which the current focus became active | monotonic per focus row |
| focus_owner_session_id | ToolSessionId | required | session that currently owns the focus row | must resolve through tool-session publications |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_focus_refs`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
