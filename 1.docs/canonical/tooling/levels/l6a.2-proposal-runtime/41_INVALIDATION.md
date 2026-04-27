# Invalidation

This contract belongs specifically to the proposal runtime level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `proposal_runtime`
- dependency shift in `l6a.0-assistant-sessions` that changes `proposal_runtime_id` semantics
- dependency shift in `l6.9-budget-runtime` that changes `source_session_id` semantics
- dependency shift in `l6.7-stream-plane` that changes `state` semantics
- supersede, deny, or cancel affecting `proposal_runtime_id`
- budget pressure that invalidates disposable outputs of `proposal_runtime` while preserving authoritative rows

## Invalidation law
Invalidation in `proposal_runtime` must explicitly name stale records such as `proposal_runtime_id`, `source_session_id`, `state`, `source_digest` instead of rebuilding an unnamed mirror.
