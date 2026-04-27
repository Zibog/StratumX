# Invalidation And Refresh

This contract belongs specifically to the viewport system editor level and is expected to become direct implementation work.


## Refresh triggers for `viewport_system`
- dependency change in tooling preview_runtime that affects `viewport_id`
- dependency change in world/scene suites that affects `camera_state_ref`
- dependency change in overlay and gizmo system that affects `navigation_mode`
- explicit user action changing `viewport_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `viewport_system`

## Invalidation law
Refresh must name stale records such as `viewport_id`, `camera_state_ref`, `navigation_mode`, `preview_frame_ref` rather than silently rebuilding hidden mirrors.
