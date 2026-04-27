# Invalidation And Refresh

This contract belongs specifically to the workspace layout system editor level and is expected to become direct implementation work.


## Refresh triggers for `workspace_layout_system`
- dependency change in editor shell that affects `workspace_layout_id`
- dependency change in workspace schema and migration model that affects `dock_tree_ref`
- dependency change in autosave recovery model that affects `saved_layout_slot`
- explicit user action changing `workspace_layout_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `workspace_layout_system`

## Invalidation law
Refresh must name stale records such as `workspace_layout_id`, `dock_tree_ref`, `saved_layout_slot`, `migration_state` rather than silently rebuilding hidden mirrors.
