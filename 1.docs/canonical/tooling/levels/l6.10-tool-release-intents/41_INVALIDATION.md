# Invalidation

This contract belongs specifically to the tool release intents level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_release_intents`
- dependency shift in `l6.0-authority-core` that changes `release_intent_id` semantics
- dependency shift in `l6.0-tool-session` that changes `intent_kind` semantics
- supersede, deny, or cancel affecting `release_intent_id`
- budget pressure that invalidates disposable outputs of `tool_release_intents` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_release_intents` must explicitly name stale records such as `release_intent_id`, `intent_kind`, `target_channel`, `artifact_scope` instead of rebuilding an unnamed mirror.
