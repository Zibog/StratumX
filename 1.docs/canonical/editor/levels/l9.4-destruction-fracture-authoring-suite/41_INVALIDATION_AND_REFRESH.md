# Invalidation And Refresh

This contract belongs specifically to the destruction fracture authoring suite editor level and is expected to become direct implementation work.


## Refresh triggers for `destruction_fracture_authoring_suite`
- dependency change in viewport system that affects `destruction_suite_session_id`
- dependency change in inspector system that affects `fracture_target_ref`
- dependency change in tooling fracture/destruction families that affects `fracture_pattern_state`
- explicit user action changing `destruction_suite_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `destruction_fracture_authoring_suite`

## Invalidation law
Refresh must name stale records such as `destruction_suite_session_id`, `fracture_target_ref`, `fracture_pattern_state`, `destruction_preview_ref` rather than silently rebuilding hidden mirrors.
