# Communication

## Ingress
- focus-change publications from inspector or viewport
- pin/unpin focus actions
- focus clear or retarget updates

## Egress
- stable focus refs keyed by focus_ref_id
- focus owner surface descriptors
- focus epoch notifications

## Communication law
`tool_focus_refs` is publication-oriented. It may ingest bounded upstream signals, normalize them into typed sidecar rows, and emit only the declared publication classes above. Direct mutation authority, hidden callbacks, or implicit UI ownership are illegal.
