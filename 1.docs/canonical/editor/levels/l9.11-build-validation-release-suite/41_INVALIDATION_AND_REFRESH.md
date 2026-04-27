# Invalidation And Refresh

This contract belongs specifically to the build validation release suite editor level and is expected to become direct implementation work.


## Refresh triggers for `build_validation_release_suite`
- dependency change in build-release surface that affects `bvr_suite_session_id`
- dependency change in diagnostics surface that affects `validation_graph_ref`
- dependency change in tooling build/release/validation families that affects `bake_queue_ref`
- explicit user action changing `bvr_suite_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `build_validation_release_suite`

## Invalidation law
Refresh must name stale records such as `bvr_suite_session_id`, `validation_graph_ref`, `bake_queue_ref`, `release_readiness_ref` rather than silently rebuilding hidden mirrors.
