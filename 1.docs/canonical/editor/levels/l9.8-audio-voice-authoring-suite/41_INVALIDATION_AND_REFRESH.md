# Invalidation And Refresh

This contract belongs specifically to the audio voice authoring suite editor level and is expected to become direct implementation work.


## Refresh triggers for `audio_voice_authoring_suite`
- dependency change in content browser that affects `audio_suite_session_id`
- dependency change in timeline surface that affects `audio_target_ref`
- dependency change in tooling audio/acoustics families that affects `emitter_state`
- explicit user action changing `audio_suite_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `audio_voice_authoring_suite`

## Invalidation law
Refresh must name stale records such as `audio_suite_session_id`, `audio_target_ref`, `emitter_state`, `dialogue_binding_ref` rather than silently rebuilding hidden mirrors.
