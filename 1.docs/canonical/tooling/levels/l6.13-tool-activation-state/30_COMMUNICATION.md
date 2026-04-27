# Communication

## Ingress
- evaluated activation rules
- workspace capability changes
- tool enable/disable requests

## Egress
- activation state rows keyed by activation_state_id
- resolved rule links
- state epoch publications

## Communication law
`tool_activation_state` is publication-oriented. It may ingest bounded upstream signals, normalize them into typed sidecar rows, and emit only the declared publication classes above. Direct mutation authority, hidden callbacks, or implicit UI ownership are illegal.
