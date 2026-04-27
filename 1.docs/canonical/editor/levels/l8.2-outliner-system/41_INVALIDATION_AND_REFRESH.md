# Invalidation And Refresh

This contract belongs specifically to the outliner system editor level and is expected to become direct implementation work.


## Refresh triggers for `outliner_system`
- dependency change in scene-entity suite that affects `outliner_view_id`
- dependency change in world suite that affects `hierarchy_projection_ref`
- dependency change in content browser that affects `row_state_set`
- dependency change in inspector system that affects `filter_state`
- explicit user action changing `outliner_view_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `outliner_system`

## Invalidation law
Refresh must name stale records such as `outliner_view_id`, `hierarchy_projection_ref`, `row_state_set`, `filter_state` rather than silently rebuilding hidden mirrors.
