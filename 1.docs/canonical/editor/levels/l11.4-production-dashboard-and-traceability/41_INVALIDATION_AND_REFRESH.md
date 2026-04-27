# Invalidation And Refresh

This contract belongs specifically to the production dashboard and traceability editor level and is expected to become direct implementation work.


## Refresh triggers for `production_dashboard_and_traceability`
- dependency change in diagnostics surface that affects `dashboard_surface_id`
- dependency change in build-release surface that affects `traceability_view_ref`
- dependency change in tooling reporting/meta families that affects `milestone_summary_ref`
- explicit user action changing `dashboard_surface_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `production_dashboard_and_traceability`

## Invalidation law
Refresh must name stale records such as `dashboard_surface_id`, `traceability_view_ref`, `milestone_summary_ref`, `filter_state` rather than silently rebuilding hidden mirrors.
