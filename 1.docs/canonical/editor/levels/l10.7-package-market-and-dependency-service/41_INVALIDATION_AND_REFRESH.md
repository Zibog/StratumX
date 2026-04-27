# Invalidation And Refresh

This contract belongs specifically to the package market and dependency service editor level and is expected to become direct implementation work.


## Refresh triggers for `package_market_and_dependency_service`
- dependency change in content browser that affects `package_service_session_id`
- dependency change in build-release surface that affects `package_listing_ref`
- dependency change in tooling pack-release families that affects `dependency_graph_ref`
- explicit user action changing `package_service_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `package_market_and_dependency_service`

## Invalidation law
Refresh must name stale records such as `package_service_session_id`, `package_listing_ref`, `dependency_graph_ref`, `mount_plan_ref` rather than silently rebuilding hidden mirrors.
