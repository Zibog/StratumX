# Communication

## Ingress
- tool registration
- package/feature activation changes
- workspace capability changes
- safety/deny rule publications

## Egress
- activation rule rows keyed by activation_rule_id
- required surface sets
- deny condition sets
- priority publications

## Communication law
`tool_activation_rules` is publication-oriented. It may ingest bounded upstream signals, normalize them into typed sidecar rows, and emit only the declared publication classes above. Direct mutation authority, hidden callbacks, or implicit UI ownership are illegal.
