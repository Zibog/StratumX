# tool_activation_state internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| activation_state_row_id | ActivationStateRowId | required | canonical storage row identity | stable within the sidecar store |
| activation_state_id | ActivationStateId | required | foreign key to the published activation state | must resolve to a root-level row |
| entered_at_cursor | Cursor | required | cursor when the current activation state was entered | monotonic per activation row |
| deactivation_reason | DeactivationReason | optional | explicit reason for leaving the active state | required before terminal deactivation when not user-cancelled |
| budget_binding_ref | BudgetTicketRef | optional | budget ticket bound to the active state | present only when activation consumes reserved budget |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_activation_state`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
