# Invalidation

This contract belongs specifically to the tool activation state level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_activation_state`
- dependency shift in `l6.0-authority-core` that changes `activation_state_id` semantics
- dependency shift in `l6.0-tool-session` that changes `tool_kind` semantics
- supersede, deny, or cancel affecting `activation_state_id`
- budget pressure that invalidates disposable outputs of `tool_activation_state` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_activation_state` must explicitly name stale records such as `activation_state_id`, `tool_kind`, `state`, `resolved_rule_id` instead of rebuilding an unnamed mirror.
