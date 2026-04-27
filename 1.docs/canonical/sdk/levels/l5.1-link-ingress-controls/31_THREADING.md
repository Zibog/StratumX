# Threading

## Concurrency posture for link ingress controls
- publication and mutation of ingress_control_envelope_id remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as ingress_control_envelope_id, control_kind, target_object_handle without creating a second writer
- no background task may rewrite already-published link ingress controls rows

## Forbidden concurrency patterns
- hidden mutable mirrors of link ingress controls
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
