# Invalidation And Refresh

This contract belongs specifically to the material lookdev authoring suite editor level and is expected to become direct implementation work.


## Refresh triggers for `material_lookdev_authoring_suite`
- dependency change in content browser that affects `lookdev_suite_session_id`
- dependency change in viewport system that affects `material_target_ref`
- dependency change in import-export pipeline that affects `slot_mapping_state`
- dependency change in tooling material/render families that affects `lookdev_preview_ref`
- explicit user action changing `lookdev_suite_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `material_lookdev_authoring_suite`

## Invalidation law
Refresh must name stale records such as `lookdev_suite_session_id`, `material_target_ref`, `slot_mapping_state`, `lookdev_preview_ref` rather than silently rebuilding hidden mirrors.
