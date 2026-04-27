# Communication

## Ingress
- diagnostics event streams
- filter/grouping change requests
- issue refresh triggers from build or validation

## Egress
- diagnostic view ids and issue-set refs
- filter digest publications
- freshness epoch updates

## Communication law
`tool_diagnostics_views` is publication-oriented. It may ingest bounded upstream signals, normalize them into typed sidecar rows, and emit only the declared publication classes above. Direct mutation authority, hidden callbacks, or implicit UI ownership are illegal.
