# Communication

## Ingress
- validation findings
- build/release errors and warnings
- runtime bridge fault publications
- preview failure summaries

## Egress
- diagnostic event rows keyed by diagnostic_event_id
- severity and scope publications
- ordered issue streams

## Communication law
`tool_diagnostics_events` is publication-oriented. It may ingest bounded upstream signals, normalize them into typed sidecar rows, and emit only the declared publication classes above. Direct mutation authority, hidden callbacks, or implicit UI ownership are illegal.
