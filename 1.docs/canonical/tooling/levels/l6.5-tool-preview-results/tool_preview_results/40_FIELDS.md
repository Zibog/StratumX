# tool_preview_results internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| preview_result_row_id | PreviewResultRowId | required | canonical storage row identity | stable within the sidecar store |
| preview_result_id | PreviewResultId | required | foreign key to the published result | must resolve to a root-level row |
| renderer_snapshot_ref | SnapshotRef | optional | renderer/runtime snapshot used to produce the result | required for snapshot-backed results |
| failure_reason | PreviewFailureReason | optional | explicit failure/degradation reason | required when result state is failed or degraded |
| retention_policy | PreviewRetentionPolicy | required | retention/eviction posture for the result row | must use declared enum |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_preview_results`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
