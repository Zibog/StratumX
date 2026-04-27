# Invalidation

This contract belongs specifically to the canon reasoner level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `canon_reasoner`
- dependency shift in `l6.3-snapshot-plane` that changes `canon_reasoner_record_id` semantics
- dependency shift in `l6.4-index-plane` that changes `scope_id` semantics
- dependency shift in `l6.5-derived-plane` that changes `state` semantics
- dependency shift in `l6.6-artifact-plane` that changes `input_ref_set` semantics
- supersede, deny, or cancel affecting `canon_reasoner_record_id`
- budget pressure that invalidates disposable outputs of `canon_reasoner` while preserving authoritative rows

## Invalidation law
Invalidation in `canon_reasoner` must explicitly name stale records such as `canon_reasoner_record_id`, `scope_id`, `state`, `input_ref_set` instead of rebuilding an unnamed mirror.
