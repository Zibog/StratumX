# Published tool-session fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| tool_session_id | ToolSessionId | required | public session identity | stable across all publications about the same session |
| session_origin | SessionOrigin | required | origin class for the session such as editor, assistant, automation, or recovery | must use declared enum |
| attachment_scope | ScopeRef | required | public scope binding for the session | must resolve through declared public refs |
| session_state | ToolSessionState | required | externally visible lifecycle state | must transition monotonically |
| opened_at_cursor | OpenedAtCursor | required | cursor when the session became visible to consumers | monotonic per session |
| closed_at_cursor | ClosedAtCursor | optional | cursor when the session closed or drained | absent until closure begins |

## Publication law
This file freezes the externally visible publication contract for tool sessions. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.
