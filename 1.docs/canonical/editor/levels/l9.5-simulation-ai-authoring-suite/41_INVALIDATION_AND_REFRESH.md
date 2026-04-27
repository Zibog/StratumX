# Invalidation And Refresh

This contract belongs specifically to the simulation ai authoring suite editor level and is expected to become direct implementation work.


## Refresh triggers for `simulation_ai_authoring_suite`
- dependency change in viewport system that affects `simulation_suite_session_id`
- dependency change in inspector system that affects `sim_target_scope`
- dependency change in quest-event-logic suite that affects `ai_profile_ref`
- dependency change in tooling population/combat/simulation families that affects `beat_preview_ref`
- explicit user action changing `simulation_suite_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `simulation_ai_authoring_suite`

## Invalidation law
Refresh must name stale records such as `simulation_suite_session_id`, `sim_target_scope`, `ai_profile_ref`, `beat_preview_ref` rather than silently rebuilding hidden mirrors.
