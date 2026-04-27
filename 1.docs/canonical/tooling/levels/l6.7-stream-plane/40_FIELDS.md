# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| stream_event_id | StreamEventId | stable event identity | unique within one stream |
| stream_kind | StreamKind | closed stream class enum | must use declared enum |
| source_scope_id | SourceScopeId | scope that emitted the event | explicit and bounded |
| ordering_cursor | StreamOrderingCursor | cursor used to preserve causal order | monotonic per stream |
| retention_window | RetentionWindow | bounded retention class for observers | finite and explicit |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `stream_plane` without consulting a hidden mirror.
