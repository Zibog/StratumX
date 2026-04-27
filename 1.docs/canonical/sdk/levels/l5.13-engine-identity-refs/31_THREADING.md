# Threading

## Concurrency posture for engine identity refs
- publication and mutation of identity_ref remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as identity_ref, identity_class, external_name without creating a second writer
- no background task may rewrite already-published engine identity refs rows

## Forbidden concurrency patterns
- hidden mutable mirrors of engine identity refs
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
