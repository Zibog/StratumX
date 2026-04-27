# Invalidation And Refresh

This contract belongs specifically to the assistant surface editor level and is expected to become direct implementation work.


## Refresh triggers for `assistant_surface`
- dependency change in tooling assistant runtimes that affects `assistant_surface_id`
- dependency change in plugin host that affects `conversation_session_id`
- dependency change in diagnostics surface that affects `proposal_view_ref`
- explicit user action changing `assistant_surface_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `assistant_surface`

## Invalidation law
Refresh must name stale records such as `assistant_surface_id`, `conversation_session_id`, `proposal_view_ref`, `apply_revert_action_set` rather than silently rebuilding hidden mirrors.
