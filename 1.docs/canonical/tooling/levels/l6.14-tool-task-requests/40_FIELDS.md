# Published tool-task-request fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| tool_task_request_id | ToolTaskRequestId | required | published identity for a tool task request | stable for request/result correlation |
| request_kind | ToolTaskRequestKind | required | kind of task being requested | must use declared enum |
| issuer_session_id | ToolSessionId | required | session that issued the task request | must resolve through tool-session publications |
| target_scope | ScopeRef | required | scope the task request addresses | must resolve through declared refs |
| priority_band | TaskPriorityBand | optional | priority band for the task request | must use declared enum when present |
| requested_deadline_cursor | Cursor | optional | deadline cursor for latency-sensitive tasks | must be >= request cursor when present |

## Publication law
This file freezes the externally visible publication contract for tool task requests. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.
