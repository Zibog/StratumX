# Communication

## Ingress
- tool open/attach/detach lifecycle signals
- session bootstrap from editor attachment
- session close/drain updates from orchestration

## Egress
- published session facts keyed by tool_session_id
- session-state change notifications
- scope snapshots for replay and auditing

## Communication law
`tool_session` is publication-oriented. It may ingest bounded upstream signals, normalize them into typed sidecar rows, and emit only the declared publication classes above. Direct mutation authority, hidden callbacks, or implicit UI ownership are illegal.
