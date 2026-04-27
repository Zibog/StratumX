# Invalidation

This contract belongs specifically to the tool session level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `tool_session`
- dependency shift in `l6.0-authority-core` that changes `tool_session_id` semantics
- dependency shift in `l6.0-tool-session` that changes `session_origin` semantics
- supersede, deny, or cancel affecting `tool_session_id`
- budget pressure that invalidates disposable outputs of `tool_session` while preserving authoritative rows

## Invalidation law
Invalidation in `tool_session` must explicitly name stale records such as `tool_session_id`, `session_origin`, `attachment_scope`, `session_state` instead of rebuilding an unnamed mirror.
