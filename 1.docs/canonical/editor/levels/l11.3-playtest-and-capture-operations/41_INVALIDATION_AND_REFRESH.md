# Invalidation And Refresh

This contract belongs specifically to the playtest and capture operations editor level and is expected to become direct implementation work.


## Refresh triggers for `playtest_and_capture_operations`
- dependency change in viewport system that affects `playtest_session_id`
- dependency change in diagnostics surface that affects `runtime_bind_ref`
- dependency change in build-release surface that affects `capture_request_ref`
- dependency change in tooling preview/stream/validation families that affects `runtime_watch_ref`
- explicit user action changing `playtest_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `playtest_and_capture_operations`

## Invalidation law
Refresh must name stale records such as `playtest_session_id`, `runtime_bind_ref`, `capture_request_ref`, `runtime_watch_ref` rather than silently rebuilding hidden mirrors.
