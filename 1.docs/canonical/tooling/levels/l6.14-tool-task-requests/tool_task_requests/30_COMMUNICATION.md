# tool_task_requests communication

Ingress:
- explicit tool task launches
- automation batch fan-out
- assistant approved actions

Egress:
- task request rows keyed by task_request_id
- priority and task-kind publications
- target scope mirrors

Communication law:
the sidecar is append-only publication traffic, never hidden authority mutation.
