# Invalidation And Refresh

This contract belongs specifically to the collaboration session surface editor level and is expected to become direct implementation work.


## Refresh triggers for `collaboration_session_surface`
- dependency change in tooling project/reporting families that affects `collab_surface_id`
- dependency change in review surface that affects `session_presence_ref`
- dependency change in production dashboard that affects `participant_set_ref`
- explicit user action changing `collab_surface_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `collaboration_session_surface`

## Invalidation law
Refresh must name stale records such as `collab_surface_id`, `session_presence_ref`, `participant_set_ref`, `permission_view_ref` rather than silently rebuilding hidden mirrors.
