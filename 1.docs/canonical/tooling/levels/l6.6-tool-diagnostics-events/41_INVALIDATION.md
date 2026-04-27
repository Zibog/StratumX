# Invalidation

This contract belongs specifically to the tool diagnostics events level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_diagnostics_events`
- dependency shift in `l6.0-authority-core` that changes `diagnostic_event_id` semantics
- dependency shift in `l6.0-tool-session` that changes `diagnostic_kind` semantics
- supersede, deny, or cancel affecting `diagnostic_event_id`
- budget pressure that invalidates disposable outputs of `tool_diagnostics_events` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_diagnostics_events` must explicitly name stale records such as `diagnostic_event_id`, `diagnostic_kind`, `severity`, `source_scope_id` instead of rebuilding an unnamed mirror.
