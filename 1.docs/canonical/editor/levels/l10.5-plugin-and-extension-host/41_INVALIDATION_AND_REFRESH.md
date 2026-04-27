# Invalidation And Refresh

This contract belongs specifically to the plugin and extension host editor level and is expected to become direct implementation work.


## Refresh triggers for `plugin_and_extension_host`
- dependency change in editor shell that affects `plugin_host_session_id`
- dependency change in inspector system that affects `registered_dock_set`
- dependency change in assistant surface that affects `inspector_renderer_set`
- dependency change in tooling assistant/validation families that affects `extension_capability_set`
- explicit user action changing `plugin_host_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `plugin_and_extension_host`

## Invalidation law
Refresh must name stale records such as `plugin_host_session_id`, `registered_dock_set`, `inspector_renderer_set`, `extension_capability_set` rather than silently rebuilding hidden mirrors.
