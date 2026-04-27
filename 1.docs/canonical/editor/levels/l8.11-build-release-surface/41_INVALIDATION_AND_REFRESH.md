# Invalidation And Refresh

This contract belongs specifically to the build release surface editor level and is expected to become direct implementation work.


## Refresh triggers for `build_release_surface`
- dependency change in tooling build_runtime that affects `build_release_surface_id`
- dependency change in tooling release_runtime that affects `build_status_view_ref`
- dependency change in package/dependency service that affects `release_manifest_ref`
- dependency change in asset gate and approval surface that affects `action_availability_set`
- explicit user action changing `build_release_surface_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `build_release_surface`

## Invalidation law
Refresh must name stale records such as `build_release_surface_id`, `build_status_view_ref`, `release_manifest_ref`, `action_availability_set` rather than silently rebuilding hidden mirrors.
