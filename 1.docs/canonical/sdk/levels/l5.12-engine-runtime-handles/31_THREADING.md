# Threading

## Concurrency posture for engine runtime handles
- publication and mutation of runtime_handle remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as runtime_handle, runtime_class, session_handle without creating a second writer
- no background task may rewrite already-published engine runtime handles rows

## Forbidden concurrency patterns
- hidden mutable mirrors of engine runtime handles
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
