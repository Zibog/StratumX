# tool_session communication

Ingress:
- tool open/attach/detach lifecycle signals
- session bootstrap from editor attachment
- session close/drain updates from orchestration

Egress:
- published session facts keyed by tool_session_id
- session-state change notifications
- scope snapshots for replay and auditing

Communication law:
the sidecar is append-only publication traffic, never hidden authority mutation.
