# Invalidation And Refresh

This contract belongs specifically to the template preset and scaffold service editor level and is expected to become direct implementation work.


## Refresh triggers for `template_preset_and_scaffold_service`
- dependency change in project bootstrap service that affects `scaffold_request_id`
- dependency change in content browser that affects `template_ref`
- dependency change in tooling content/meta families that affects `target_scope`
- explicit user action changing `scaffold_request_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `template_preset_and_scaffold_service`

## Invalidation law
Refresh must name stale records such as `scaffold_request_id`, `template_ref`, `target_scope`, `generated_item_set` rather than silently rebuilding hidden mirrors.
