# Published diagnostics-event fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| diagnostics_event_id | DiagnosticsEventId | required | published diagnostics event identity | stable for event correlation |
| event_kind | DiagnosticsEventKind | required | kind of diagnostics event | must use declared enum |
| subject_ref | SubjectRef | optional | subject implicated by the event | required when the event targets a concrete subject |
| severity | DiagnosticsSeverity | required | severity band of the event | must use declared enum |
| emitted_at_cursor | Cursor | required | cursor when the event was emitted | monotonic within the diagnostics stream |
| origin_service | DiagnosticsOriginService | required | service/runtime that emitted the event | must use declared enum |

## Publication law
This file freezes the externally visible publication contract for diagnostics events. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.
