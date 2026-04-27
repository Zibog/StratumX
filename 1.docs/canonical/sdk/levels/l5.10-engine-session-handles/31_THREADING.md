# Threading

## Concurrency posture for engine session handles
- publication and mutation of session_handle remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as session_handle, session_scope_id, origin_class without creating a second writer
- no background task may rewrite already-published engine session handles rows

## Forbidden concurrency patterns
- hidden mutable mirrors of engine session handles
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
