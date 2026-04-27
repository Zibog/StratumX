# Invalidation

This contract belongs specifically to the safety gates level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `safety_gates`
- dependency shift in `l6a.0-assistant-sessions` that changes `safety_gate_id` semantics
- dependency shift in `l6.9-budget-runtime` that changes `source_session_id` semantics
- dependency shift in `l6.7-stream-plane` that changes `state` semantics
- supersede, deny, or cancel affecting `safety_gate_id`
- budget pressure that invalidates disposable outputs of `safety_gates` while preserving authoritative rows

## Invalidation law
Invalidation in `safety_gates` must explicitly name stale records such as `safety_gate_id`, `source_session_id`, `state`, `source_digest` instead of rebuilding an unnamed mirror.
