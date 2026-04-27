# Published tool-task-result fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| tool_task_result_id | ToolTaskResultId | required | published identity for a tool task result | stable for result correlation |
| tool_task_request_id | ToolTaskRequestId | required | originating task request | must resolve through published request rows |
| result_state | ToolTaskResultState | required | state of the task result | must use declared enum |
| result_payload_ref | PayloadRef | optional | payload or artifact produced by the task | required when the result is successful |
| result_cursor | Cursor | required | cursor when the result became visible | monotonic within the task result stream |
| failure_reason | TaskFailureReason | optional | explicit reason for failure or downgrade | required when result state is failed or degraded |

## Publication law
This file freezes the externally visible publication contract for tool task results. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.
