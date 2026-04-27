# Threading

This contract belongs specifically to the weather environment authoring suite editor level and is expected to become direct implementation work.


## Threading posture for `weather_environment_authoring_suite`
- interactive ownership of `environment_suite_session_id`, `environment_target_scope` stays on the editor interaction thread unless a declared background runtime owns the work
- long-running work related to weather profile updates; lighting/weather preview refreshes publishes progress through bounded request/result or stream surfaces
- visible state transitions for `{key}` remain serializable for undo/redo and audit

## Operational note
This file remains active and package-specific for `l9.6-weather-environment-authoring-suite` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
