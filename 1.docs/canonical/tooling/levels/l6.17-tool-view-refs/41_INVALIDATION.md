# Invalidation

This contract belongs specifically to the tool view refs level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_view_refs`
- dependency shift in `l6.0-authority-core` that changes `view_ref_id` semantics
- dependency shift in `l6.0-tool-session` that changes `view_kind` semantics
- supersede, deny, or cancel affecting `view_ref_id`
- budget pressure that invalidates disposable outputs of `tool_view_refs` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_view_refs` must explicitly name stale records such as `view_ref_id`, `view_kind`, `target_scope`, `hosting_surface` instead of rebuilding an unnamed mirror.
