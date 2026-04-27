# Invalidation And Refresh

This contract belongs specifically to the quest event logic authoring suite editor level and is expected to become direct implementation work.


## Refresh triggers for `quest_event_logic_authoring_suite`
- dependency change in graph authoring service that affects `logic_suite_session_id`
- dependency change in timeline surface that affects `quest_graph_ref`
- dependency change in tooling simulation/content families that affects `signal_track_ref`
- explicit user action changing `logic_suite_session_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `quest_event_logic_authoring_suite`

## Invalidation law
Refresh must name stale records such as `logic_suite_session_id`, `quest_graph_ref`, `signal_track_ref`, `logic_preview_ref` rather than silently rebuilding hidden mirrors.
