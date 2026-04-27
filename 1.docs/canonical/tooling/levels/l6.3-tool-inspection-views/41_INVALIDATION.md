# Invalidation

This contract belongs specifically to the tool inspection views level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_inspection_views`
- dependency shift in `l6.0-authority-core` that changes `inspection_view_id` semantics
- dependency shift in `l6.0-tool-session` that changes `target_ref` semantics
- supersede, deny, or cancel affecting `inspection_view_id`
- budget pressure that invalidates disposable outputs of `tool_inspection_views` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_inspection_views` must explicitly name stale records such as `inspection_view_id`, `target_ref`, `field_set_ref`, `view_generation_epoch` instead of rebuilding an unnamed mirror.
