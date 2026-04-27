# Invalidation And Refresh

This contract belongs specifically to the graph authoring service editor level and is expected to become direct implementation work.


## Refresh triggers for `graph_authoring_service`
- dependency change in content browser that affects `graph_editor_session_id`
- dependency change in quest-event-logic suite that affects `graph_target_ref`
- dependency change in ui suite that affects `node_selection_ref_set`
- dependency change in plugin host that affects `schema_binding_ref`
- explicit user action changing `graph_editor_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `graph_authoring_service`

## Invalidation law
Refresh must name stale records such as `graph_editor_session_id`, `graph_target_ref`, `node_selection_ref_set`, `schema_binding_ref` rather than silently rebuilding hidden mirrors.
