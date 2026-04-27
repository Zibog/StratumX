# Invalidation And Refresh

This contract belongs specifically to the inspector system editor level and is expected to become direct implementation work.


## Refresh triggers for `inspector_system`
- dependency change in scene-entity suite that affects `inspector_view_id`
- dependency change in content browser that affects `inspected_target_ref`
- dependency change in validation_runtime that affects `component_editor_set`
- dependency change in plugin-and-extension-host that affects `header_state`
- explicit user action changing `inspector_view_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `inspector_system`

## Invalidation law
Refresh must name stale records such as `inspector_view_id`, `inspected_target_ref`, `component_editor_set`, `header_state` rather than silently rebuilding hidden mirrors.
