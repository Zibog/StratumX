# Invalidation And Refresh

This contract belongs specifically to the diagnostics surface editor level and is expected to become direct implementation work.


## Refresh triggers for `diagnostics_surface`
- dependency change in tooling diagnostics events/views that affects `diagnostics_surface_id`
- dependency change in validation runtime that affects `issue_view_ref`
- dependency change in production dashboard that affects `filter_state`
- explicit user action changing `diagnostics_surface_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `diagnostics_surface`

## Invalidation law
Refresh must name stale records such as `diagnostics_surface_id`, `issue_view_ref`, `filter_state`, `selection_ref` rather than silently rebuilding hidden mirrors.
