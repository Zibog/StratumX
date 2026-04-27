# Published preview-result fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| preview_result_id | PreviewResultId | required | published identity for a preview result | stable for result correlation |
| preview_request_id | PreviewRequestId | required | originating preview request | must resolve through published request rows |
| result_state | PreviewResultState | required | state of the preview result | must use declared enum |
| artifact_ref | ArtifactRef | optional | artifact or image/view product produced by the preview | required when the result is successful |
| result_cursor | Cursor | required | cursor when the result became visible | monotonic within the preview result stream |
| degradation_band | PreviewQualityBand | optional | actual quality band used to produce the preview | must use declared enum when present |

## Publication law
This file freezes the externally visible publication contract for preview results. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.
