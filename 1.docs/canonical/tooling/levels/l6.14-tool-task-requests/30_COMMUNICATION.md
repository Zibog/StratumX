# Communication

## Ingress
- explicit tool task launches
- automation batch fan-out
- assistant approved actions
- editor build/validate requests

## Egress
- task request rows keyed by task_request_id
- priority and task-kind publications
- target scope mirrors

## Communication law
`tool_task_requests` is publication-oriented. It may ingest bounded upstream signals, normalize them into typed sidecar rows, and emit only the declared publication classes above. Direct mutation authority, hidden callbacks, or implicit UI ownership are illegal.
