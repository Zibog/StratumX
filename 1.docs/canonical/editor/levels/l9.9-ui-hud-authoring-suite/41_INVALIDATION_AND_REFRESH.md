# Invalidation And Refresh

This contract belongs specifically to the ui hud authoring suite editor level and is expected to become direct implementation work.


## Refresh triggers for `ui_hud_authoring_suite`
- dependency change in viewport system that affects `ui_suite_session_id`
- dependency change in graph authoring service that affects `ui_target_ref`
- dependency change in tooling ui families that affects `layout_preview_ref`
- explicit user action changing `ui_suite_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `ui_hud_authoring_suite`

## Invalidation law
Refresh must name stale records such as `ui_suite_session_id`, `ui_target_ref`, `layout_preview_ref`, `binding_graph_ref` rather than silently rebuilding hidden mirrors.
