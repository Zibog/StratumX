# Invalidation And Refresh

This contract belongs specifically to the weather environment authoring suite editor level and is expected to become direct implementation work.


## Refresh triggers for `weather_environment_authoring_suite`
- dependency change in viewport system that affects `environment_suite_session_id`
- dependency change in timeline surface that affects `environment_target_scope`
- dependency change in tooling fluid-fire-weather families that affects `weather_profile_ref`
- explicit user action changing `environment_suite_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `weather_environment_authoring_suite`

## Invalidation law
Refresh must name stale records such as `environment_suite_session_id`, `environment_target_scope`, `weather_profile_ref`, `lighting_track_ref` rather than silently rebuilding hidden mirrors.
