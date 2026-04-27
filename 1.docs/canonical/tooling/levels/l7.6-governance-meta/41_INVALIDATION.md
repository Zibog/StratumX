# Invalidation

This contract belongs specifically to the governance meta level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `governance_meta`
- dependency shift in `l6.3-snapshot-plane` that changes `governance_meta_record_id` semantics
- dependency shift in `l6.4-index-plane` that changes `scope_id` semantics
- dependency shift in `l6.5-derived-plane` that changes `state` semantics
- dependency shift in `l6.6-artifact-plane` that changes `input_ref_set` semantics
- supersede, deny, or cancel affecting `governance_meta_record_id`
- budget pressure that invalidates disposable outputs of `governance_meta` while preserving authoritative rows

## Invalidation law
Invalidation in `governance_meta` must explicitly name stale records such as `governance_meta_record_id`, `scope_id`, `state`, `input_ref_set` instead of rebuilding an unnamed mirror.
