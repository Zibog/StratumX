# Invalidation

This contract belongs specifically to the assistant sessions level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `assistant_sessions`
- dependency shift in `l6a.0-assistant-sessions` that changes `assistant_session_id` semantics
- dependency shift in `l6.9-budget-runtime` that changes `source_session_id` semantics
- dependency shift in `l6.7-stream-plane` that changes `state` semantics
- supersede, deny, or cancel affecting `assistant_session_id`
- budget pressure that invalidates disposable outputs of `assistant_sessions` while preserving authoritative rows

## Invalidation law
Invalidation in `assistant_sessions` must explicitly name stale records such as `assistant_session_id`, `source_session_id`, `state`, `source_digest` instead of rebuilding an unnamed mirror.
