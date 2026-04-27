# Invalidation And Refresh

This contract belongs specifically to the overlay and gizmo system editor level and is expected to become direct implementation work.


## Refresh triggers for `overlay_and_gizmo_system`
- dependency change in viewport system that affects `overlay_set_id`
- dependency change in tool context system that affects `gizmo_state_id`
- dependency change in plugin host that affects `transform_handle_set`
- dependency change in tooling preview/stream surfaces that affects `snap_visual_state`
- explicit user action changing `overlay_set_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `overlay_and_gizmo_system`

## Invalidation law
Refresh must name stale records such as `overlay_set_id`, `gizmo_state_id`, `transform_handle_set`, `snap_visual_state` rather than silently rebuilding hidden mirrors.
