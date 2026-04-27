# tool_scene_intents communication

Ingress:
- viewport manipulations
- outliner actions
- layer/data-layer changes

Egress:
- scene intent rows keyed by scene_intent_id
- target entity-set mirrors
- requested effect publications

Communication law:
the sidecar is append-only publication traffic, never hidden authority mutation.
