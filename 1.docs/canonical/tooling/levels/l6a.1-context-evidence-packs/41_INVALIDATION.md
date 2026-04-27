# Invalidation

This contract belongs specifically to the context evidence packs level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `context_evidence_packs`
- dependency shift in `l6a.0-assistant-sessions` that changes `context_evidence_pack_id` semantics
- dependency shift in `l6.9-budget-runtime` that changes `source_session_id` semantics
- dependency shift in `l6.7-stream-plane` that changes `state` semantics
- supersede, deny, or cancel affecting `context_evidence_pack_id`
- budget pressure that invalidates disposable outputs of `context_evidence_packs` while preserving authoritative rows

## Invalidation law
Invalidation in `context_evidence_packs` must explicitly name stale records such as `context_evidence_pack_id`, `source_session_id`, `state`, `source_digest` instead of rebuilding an unnamed mirror.
