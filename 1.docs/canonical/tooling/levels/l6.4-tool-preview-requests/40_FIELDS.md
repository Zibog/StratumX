# Published preview-request fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| preview_request_id | PreviewRequestId | required | published identity for a preview request | stable across request/result correlation |
| request_kind | PreviewRequestKind | required | class of preview being requested | must use declared enum |
| target_scope | ScopeRef | required | scope to preview | must resolve through declared refs |
| issuer_session_id | ToolSessionId | required | tool session that issued the request | must resolve through tool-session publications |
| requested_quality_band | PreviewQualityBand | optional | requested preview quality/degradation band | must use declared enum when present |
| request_cursor | Cursor | required | cursor when the request entered the preview plane | monotonic within the request stream |

## Publication law
This file freezes the externally visible publication contract for preview requests. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.
