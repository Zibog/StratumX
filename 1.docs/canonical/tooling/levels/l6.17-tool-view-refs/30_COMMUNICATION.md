# Communication

## Ingress
- view open/close or attach publications
- view-target changes
- view host moves and detach/reattach operations

## Egress
- view ref rows keyed by view_ref_id
- hosting surface and target scope publications
- view epoch updates

## Communication law
`tool_view_refs` is publication-oriented. It may ingest bounded upstream signals, normalize them into typed sidecar rows, and emit only the declared publication classes above. Direct mutation authority, hidden callbacks, or implicit UI ownership are illegal.
