# Invalidation And Refresh

This contract belongs specifically to the scene entity authoring suite editor level and is expected to become direct implementation work.


## Refresh triggers for `scene_entity_authoring_suite`
- dependency change in outliner system that affects `scene_suite_session_id`
- dependency change in inspector system that affects `entity_selection_ref_set`
- dependency change in content browser that affects `prefab_instance_view_ref`
- dependency change in plugin host that affects `component_list_ref`
- explicit user action changing `scene_suite_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `scene_entity_authoring_suite`

## Invalidation law
Refresh must name stale records such as `scene_suite_session_id`, `entity_selection_ref_set`, `prefab_instance_view_ref`, `component_list_ref` rather than silently rebuilding hidden mirrors.
