# Communication

## Ingress
- viewport manipulations
- outliner actions
- layer/data-layer changes
- cell/chunk/world authoring actions

## Egress
- scene intent rows keyed by scene_intent_id
- target entity-set mirrors
- requested effect publications

## Communication law
`tool_scene_intents` is publication-oriented. It may ingest bounded upstream signals, normalize them into typed sidecar rows, and emit only the declared publication classes above. Direct mutation authority, hidden callbacks, or implicit UI ownership are illegal.
