# Invalidation

This contract belongs specifically to the tool panel refs level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_panel_refs`
- dependency shift in `l6.0-authority-core` that changes `panel_ref_id` semantics
- dependency shift in `l6.0-tool-session` that changes `panel_kind` semantics
- supersede, deny, or cancel affecting `panel_ref_id`
- budget pressure that invalidates disposable outputs of `tool_panel_refs` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_panel_refs` must explicitly name stale records such as `panel_ref_id`, `panel_kind`, `attachment_scope`, `hosting_surface` instead of rebuilding an unnamed mirror.
