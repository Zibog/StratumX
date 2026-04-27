# Invalidation And Refresh

This contract belongs specifically to the tool context system editor level and is expected to become direct implementation work.


## Refresh triggers for `tool_context_system`
- dependency change in interaction routing system that affects `tool_context_id`
- dependency change in overlay and gizmo system that affects `active_tool_kind`
- dependency change in viewport system that affects `mode_state`
- dependency change in tooling activation sidecars that affects `selection_scope`
- explicit user action changing `tool_context_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `tool_context_system`

## Invalidation law
Refresh must name stale records such as `tool_context_id`, `active_tool_kind`, `mode_state`, `selection_scope` rather than silently rebuilding hidden mirrors.
