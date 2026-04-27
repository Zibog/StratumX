# Invalidation And Refresh

This contract belongs specifically to the interaction routing system editor level and is expected to become direct implementation work.


## Refresh triggers for `interaction_routing_system`
- dependency change in viewport system that affects `interaction_route_id`
- dependency change in content browser that affects `input_event_batch_ref`
- dependency change in inspector system that affects `target_surface_ref`
- dependency change in tool context system that affects `capture_state`
- explicit user action changing `interaction_route_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `interaction_routing_system`

## Invalidation law
Refresh must name stale records such as `interaction_route_id`, `input_event_batch_ref`, `target_surface_ref`, `capture_state` rather than silently rebuilding hidden mirrors.
