# Threading

## Concurrency posture for link ingress packets
- publication and mutation of packet_id remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as packet_id, session_handle, transport_policy_id without creating a second writer
- no background task may rewrite already-published link ingress packets rows

## Forbidden concurrency patterns
- hidden mutable mirrors of link ingress packets
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
