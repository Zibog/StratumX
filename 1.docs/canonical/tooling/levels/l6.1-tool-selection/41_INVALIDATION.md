# Invalidation

This contract belongs specifically to the tool selection level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_selection`
- dependency shift in `l6.0-authority-core` that changes `selection_ref_set_id` semantics
- dependency shift in `l6.0-tool-session` that changes `selected_ref_set` semantics
- supersede, deny, or cancel affecting `selection_ref_set_id`
- budget pressure that invalidates disposable outputs of `tool_selection` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_selection` must explicitly name stale records such as `selection_ref_set_id`, `selected_ref_set`, `selection_source`, `selection_epoch` instead of rebuilding an unnamed mirror.
