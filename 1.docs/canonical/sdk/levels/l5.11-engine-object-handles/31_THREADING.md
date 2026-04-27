# Threading

## Concurrency posture for engine object handles
- publication and mutation of object_handle remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as object_handle, identity_ref, session_handle without creating a second writer
- no background task may rewrite already-published engine object handles rows

## Forbidden concurrency patterns
- hidden mutable mirrors of engine object handles
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
