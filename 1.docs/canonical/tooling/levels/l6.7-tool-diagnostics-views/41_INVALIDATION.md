# Invalidation

This contract belongs specifically to the tool diagnostics views level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_diagnostics_views`
- dependency shift in `l6.0-authority-core` that changes `diagnostic_view_id` semantics
- dependency shift in `l6.0-tool-session` that changes `issue_set_ref` semantics
- supersede, deny, or cancel affecting `diagnostic_view_id`
- budget pressure that invalidates disposable outputs of `tool_diagnostics_views` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_diagnostics_views` must explicitly name stale records such as `diagnostic_view_id`, `issue_set_ref`, `grouping_mode`, `filter_digest` instead of rebuilding an unnamed mirror.
