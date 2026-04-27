# Invalidation And Refresh

This contract belongs specifically to the project bootstrap service editor level and is expected to become direct implementation work.


## Refresh triggers for `project_bootstrap_service`
- dependency change in workspace layout system that affects `bootstrap_request_id`
- dependency change in template/preset service that affects `project_manifest_ref`
- dependency change in package/dependency service that affects `content_mount_plan_ref`
- dependency change in tooling project meta families that affects `workspace_seed_ref`
- explicit user action changing `bootstrap_request_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `project_bootstrap_service`

## Invalidation law
Refresh must name stale records such as `bootstrap_request_id`, `project_manifest_ref`, `content_mount_plan_ref`, `workspace_seed_ref` rather than silently rebuilding hidden mirrors.
