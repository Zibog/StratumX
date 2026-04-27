# Communication

## Ingress
- panel open/close publications
- dock/undock changes
- host migration or split-view changes

## Egress
- panel ref rows keyed by panel_ref_id
- hosting surface publications
- panel epoch updates

## Communication law
`tool_panel_refs` is publication-oriented. It may ingest bounded upstream signals, normalize them into typed sidecar rows, and emit only the declared publication classes above. Direct mutation authority, hidden callbacks, or implicit UI ownership are illegal.
