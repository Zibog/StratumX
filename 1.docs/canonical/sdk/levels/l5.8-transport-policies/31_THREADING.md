# Threading

## Concurrency posture for transport policies
- publication and mutation of transport_policy_id remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as transport_policy_id, framing_kind, max_payload_bytes without creating a second writer
- no background task may rewrite already-published transport policies rows

## Forbidden concurrency patterns
- hidden mutable mirrors of transport policies
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
