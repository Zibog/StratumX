# Invalidation

This contract belongs specifically to the tool activation rules level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_activation_rules`
- dependency shift in `l6.0-authority-core` that changes `activation_rule_id` semantics
- dependency shift in `l6.0-tool-session` that changes `tool_kind` semantics
- supersede, deny, or cancel affecting `activation_rule_id`
- budget pressure that invalidates disposable outputs of `tool_activation_rules` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_activation_rules` must explicitly name stale records such as `activation_rule_id`, `tool_kind`, `required_surface_set`, `deny_condition_set` instead of rebuilding an unnamed mirror.
