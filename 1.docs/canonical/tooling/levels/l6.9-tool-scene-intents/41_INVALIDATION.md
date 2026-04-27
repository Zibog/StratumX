# Invalidation

This contract belongs specifically to the tool scene intents level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_scene_intents`
- dependency shift in `l6.0-authority-core` that changes `scene_intent_id` semantics
- dependency shift in `l6.0-tool-session` that changes `intent_kind` semantics
- supersede, deny, or cancel affecting `scene_intent_id`
- budget pressure that invalidates disposable outputs of `tool_scene_intents` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_scene_intents` must explicitly name stale records such as `scene_intent_id`, `intent_kind`, `target_entity_set`, `requested_effect` instead of rebuilding an unnamed mirror.
