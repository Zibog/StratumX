# Invalidation

This contract belongs specifically to the tool preview requests level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_preview_requests`
- dependency shift in `l6.0-authority-core` that changes `preview_request_id` semantics
- dependency shift in `l6.0-tool-session` that changes `request_kind` semantics
- supersede, deny, or cancel affecting `preview_request_id`
- budget pressure that invalidates disposable outputs of `tool_preview_requests` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_preview_requests` must explicitly name stale records such as `preview_request_id`, `request_kind`, `target_ref_set`, `quality_hint` instead of rebuilding an unnamed mirror.
