# Invalidation

This contract belongs specifically to the workspace runtime level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `workspace_runtime`
- dependency shift in `l6.0-authority-core` that changes `workspace_session_id` semantics
- dependency shift in `l6.3-snapshot-plane` that changes `published_selection_ref_set` semantics
- dependency shift in `l6.7-stream-plane` that changes `published_focus_ref_set` semantics
- dependency shift in `l6.9-budget-runtime` that changes `published_panel_ref_set` semantics
- supersede, deny, or cancel affecting `workspace_session_id`
- budget pressure that invalidates disposable outputs of `workspace_runtime` while preserving authoritative rows

## Invalidation law
Invalidation in `workspace_runtime` must explicitly name stale records such as `workspace_session_id`, `published_selection_ref_set`, `published_focus_ref_set`, `published_panel_ref_set` instead of rebuilding an unnamed mirror.
