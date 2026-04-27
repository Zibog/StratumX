# Invalidation

This contract belongs specifically to the prompt understanding level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `prompt_understanding`
- dependency shift in `l6.3-snapshot-plane` that changes `prompt_understanding_record_id` semantics
- dependency shift in `l6.4-index-plane` that changes `scope_id` semantics
- dependency shift in `l6.5-derived-plane` that changes `state` semantics
- dependency shift in `l6.6-artifact-plane` that changes `input_ref_set` semantics
- supersede, deny, or cancel affecting `prompt_understanding_record_id`
- budget pressure that invalidates disposable outputs of `prompt_understanding` while preserving authoritative rows

## Invalidation law
Invalidation in `prompt_understanding` must explicitly name stale records such as `prompt_understanding_record_id`, `scope_id`, `state`, `input_ref_set` instead of rebuilding an unnamed mirror.
