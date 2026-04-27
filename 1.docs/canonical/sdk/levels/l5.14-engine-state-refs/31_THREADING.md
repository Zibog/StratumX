# Threading

## Concurrency posture for engine state refs
- publication and mutation of state_ref remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as state_ref, runtime_handle, snapshot_epoch without creating a second writer
- no background task may rewrite already-published engine state refs rows

## Forbidden concurrency patterns
- hidden mutable mirrors of engine state refs
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
