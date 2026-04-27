# Invalidation And Refresh

This contract belongs specifically to the animation cinematics authoring suite editor level and is expected to become direct implementation work.


## Refresh triggers for `animation_cinematics_authoring_suite`
- dependency change in timeline surface that affects `cinematics_suite_session_id`
- dependency change in viewport system that affects `sequence_ref`
- dependency change in plugin host that affects `track_binding_set`
- dependency change in tooling animation/cinematic families that affects `camera_rig_state`
- explicit user action changing `cinematics_suite_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `animation_cinematics_authoring_suite`

## Invalidation law
Refresh must name stale records such as `cinematics_suite_session_id`, `sequence_ref`, `track_binding_set`, `camera_rig_state` rather than silently rebuilding hidden mirrors.
