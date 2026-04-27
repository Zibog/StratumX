# Invalidation

This contract belongs specifically to the tool focus refs level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_focus_refs`
- dependency shift in `l6.0-authority-core` that changes `focus_ref_id` semantics
- dependency shift in `l6.0-tool-session` that changes `focused_target_ref` semantics
- supersede, deny, or cancel affecting `focus_ref_id`
- budget pressure that invalidates disposable outputs of `tool_focus_refs` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_focus_refs` must explicitly name stale records such as `focus_ref_id`, `focused_target_ref`, `focus_owner_surface`, `focus_epoch` instead of rebuilding an unnamed mirror.
