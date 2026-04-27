# Invalidation

This contract belongs specifically to the content campaigns level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `content_campaigns`
- dependency shift in `l6.3-snapshot-plane` that changes `content_campaigns_record_id` semantics
- dependency shift in `l6.4-index-plane` that changes `scope_id` semantics
- dependency shift in `l6.5-derived-plane` that changes `state` semantics
- dependency shift in `l6.6-artifact-plane` that changes `input_ref_set` semantics
- supersede, deny, or cancel affecting `content_campaigns_record_id`
- budget pressure that invalidates disposable outputs of `content_campaigns` while preserving authoritative rows

## Invalidation law
Invalidation in `content_campaigns` must explicitly name stale records such as `content_campaigns_record_id`, `scope_id`, `state`, `input_ref_set` instead of rebuilding an unnamed mirror.
