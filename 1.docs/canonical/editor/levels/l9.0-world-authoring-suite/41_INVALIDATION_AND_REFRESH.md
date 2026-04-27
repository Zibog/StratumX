# Invalidation And Refresh

This contract belongs specifically to the world authoring suite editor level and is expected to become direct implementation work.


## Refresh triggers for `world_authoring_suite`
- dependency change in viewport system that affects `world_suite_session_id`
- dependency change in outliner system that affects `cell_view_ref`
- dependency change in world tools that affects `data_layer_state_ref`
- dependency change in tooling world families that affects `streaming_preview_ref`
- explicit user action changing `world_suite_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `world_authoring_suite`

## Invalidation law
Refresh must name stale records such as `world_suite_session_id`, `cell_view_ref`, `data_layer_state_ref`, `streaming_preview_ref` rather than silently rebuilding hidden mirrors.
