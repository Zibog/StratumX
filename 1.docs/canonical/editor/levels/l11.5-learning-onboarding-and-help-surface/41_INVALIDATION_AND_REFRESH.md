# Invalidation And Refresh

This contract belongs specifically to the learning onboarding and help surface editor level and is expected to become direct implementation work.


## Refresh triggers for `learning_onboarding_and_help_surface`
- dependency change in assistant surface that affects `help_surface_id`
- dependency change in command palette that affects `help_topic_ref`
- dependency change in tooling reporting/meta families that affects `command_hint_set`
- explicit user action changing `help_surface_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `learning_onboarding_and_help_surface`

## Invalidation law
Refresh must name stale records such as `help_surface_id`, `help_topic_ref`, `command_hint_set`, `learning_progress_ref` rather than silently rebuilding hidden mirrors.
