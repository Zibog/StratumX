# Communication

## Ingress
- content browser actions
- asset import/reimport requests
- material/texture/audio/VFX authoring actions
- assistant suggestions

## Egress
- content intent rows keyed by content_intent_id
- requested effect publications
- target asset-set mirrors

## Communication law
`tool_content_intents` is publication-oriented. It may ingest bounded upstream signals, normalize them into typed sidecar rows, and emit only the declared publication classes above. Direct mutation authority, hidden callbacks, or implicit UI ownership are illegal.
