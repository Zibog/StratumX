# Invalidation And Refresh

This contract belongs specifically to the terrain landscape authoring suite editor level and is expected to become direct implementation work.


## Refresh triggers for `terrain_landscape_authoring_suite`
- dependency change in viewport system that affects `terrain_suite_session_id`
- dependency change in tool context system that affects `terrain_region_ref`
- dependency change in world suite that affects `brush_state`
- dependency change in tooling terrain/material families that affects `layer_paint_state`
- explicit user action changing `terrain_suite_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `terrain_landscape_authoring_suite`

## Invalidation law
Refresh must name stale records such as `terrain_suite_session_id`, `terrain_region_ref`, `brush_state`, `layer_paint_state` rather than silently rebuilding hidden mirrors.
