# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| preview_run_id | PreviewRunId | stable preview job identity | unique per request |
| preview_request_id | PreviewRequestId | request consumed by the job | must resolve through tool_preview_requests |
| preview_kind | PreviewKind | closed preview class enum | must use declared enum |
| source_snapshot_set | SnapshotSetRef | snapshots used to build the preview | must resolve through snapshot_plane |
| result_ref | PreviewResultRef | published result ref for consumers | must remain disposable |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `preview_runtime` without consulting a hidden mirror.
