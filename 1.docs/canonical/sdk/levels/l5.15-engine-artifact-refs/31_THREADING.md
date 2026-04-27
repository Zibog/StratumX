# Threading

## Concurrency posture for engine artifact refs
- publication and mutation of artifact_ref remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as artifact_ref, artifact_kind, source_runtime_handle without creating a second writer
- no background task may rewrite already-published engine artifact refs rows

## Forbidden concurrency patterns
- hidden mutable mirrors of engine artifact refs
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
