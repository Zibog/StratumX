# Threading

## Concurrency posture for link egress observations
- publication and mutation of observation_batch_id remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as observation_batch_id, source_runtime_handle, source_state_ref without creating a second writer
- no background task may rewrite already-published link egress observations rows

## Forbidden concurrency patterns
- hidden mutable mirrors of link egress observations
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
