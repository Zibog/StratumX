# Invalidation And Refresh

This contract belongs specifically to the automation and batch service editor level and is expected to become direct implementation work.


## Refresh triggers for `automation_and_batch_service`
- dependency change in plugin host that affects `batch_run_id`
- dependency change in package/dependency service that affects `batch_recipe_ref`
- dependency change in tooling automation/meta families that affects `target_scope`
- explicit user action changing `batch_run_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `automation_and_batch_service`

## Invalidation law
Refresh must name stale records such as `batch_run_id`, `batch_recipe_ref`, `target_scope`, `progress_view_ref` rather than silently rebuilding hidden mirrors.
