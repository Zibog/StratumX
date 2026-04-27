# Invalidation And Refresh

This contract belongs specifically to the script and hot reload service editor level and is expected to become direct implementation work.


## Refresh triggers for `script_and_hot_reload_service`
- dependency change in content browser that affects `hot_reload_session_id`
- dependency change in plugin host that affects `script_target_ref`
- dependency change in tooling validation/runtime/assistant families that affects `reload_safety_state`
- explicit user action changing `hot_reload_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `script_and_hot_reload_service`

## Invalidation law
Refresh must name stale records such as `hot_reload_session_id`, `script_target_ref`, `reload_safety_state`, `reload_request_ref` rather than silently rebuilding hidden mirrors.
