# Threading

## Concurrency posture for compat capabilities
- publication and mutation of capability_id remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as capability_id, capability_name, availability_profile_id without creating a second writer
- no background task may rewrite already-published compat capabilities rows

## Forbidden concurrency patterns
- hidden mutable mirrors of compat capabilities
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
