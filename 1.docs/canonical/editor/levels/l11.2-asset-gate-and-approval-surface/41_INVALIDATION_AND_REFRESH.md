# Invalidation And Refresh

This contract belongs specifically to the asset gate and approval surface editor level and is expected to become direct implementation work.


## Refresh triggers for `asset_gate_and_approval_surface`
- dependency change in build-release surface that affects `gate_surface_id`
- dependency change in package/dependency service that affects `gate_queue_ref`
- dependency change in tooling validation/release families that affects `selected_gate_item_ref`
- explicit user action changing `gate_surface_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `asset_gate_and_approval_surface`

## Invalidation law
Refresh must name stale records such as `gate_surface_id`, `gate_queue_ref`, `selected_gate_item_ref`, `approval_state_ref` rather than silently rebuilding hidden mirrors.
