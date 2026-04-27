# Invalidation And Refresh

This contract belongs specifically to the editor shell editor level and is expected to become direct implementation work.


## Refresh triggers for `editor_shell`
- dependency change in tooling workspace_runtime that affects `shell_session_id`
- dependency change in viewport system that affects `layout_schema_id`
- dependency change in workspace layout system that affects `dock_host_set`
- dependency change in assistant/diagnostics/build surfaces that affects `global_mode_state`
- explicit user action changing `shell_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `editor_shell`

## Invalidation law
Refresh must name stale records such as `shell_session_id`, `layout_schema_id`, `dock_host_set`, `global_mode_state` rather than silently rebuilding hidden mirrors.
