# Communication

## Ingress
- preview runtime completion publications
- preview cancellation completions
- preview invalidation refreshes

## Egress
- preview result refs keyed by preview_result_id
- result-state transitions
- freshness epoch updates

## Communication law
`tool_preview_results` is publication-oriented. It may ingest bounded upstream signals, normalize them into typed sidecar rows, and emit only the declared publication classes above. Direct mutation authority, hidden callbacks, or implicit UI ownership are illegal.
