# tool_task_results internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| tool_task_result_row_id | ToolTaskResultRowId | required | canonical storage row identity | stable within the sidecar store |
| tool_task_result_id | ToolTaskResultId | required | foreign key to the published task result | must resolve to a root-level row |
| worker_trace_ref | TraceRef | optional | trace or execution record for the task result | required for traced execution modes |
| retention_policy | TaskResultRetentionPolicy | required | retention/eviction posture for the result row | must use declared enum |
| applied_budget_ticket | BudgetTicketRef | optional | budget reservation consumed by the task execution | present when execution was budget-bound |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_task_results`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
