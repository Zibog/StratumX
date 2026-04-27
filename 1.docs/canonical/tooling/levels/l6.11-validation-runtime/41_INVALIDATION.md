# Invalidation

This contract belongs specifically to the validation runtime level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `validation_runtime`
- dependency shift in `l6.3-snapshot-plane` that changes `validation_run_id` semantics
- dependency shift in `l6.4-index-plane` that changes `validation_scope` semantics
- dependency shift in `l6.7-stream-plane` that changes `rule_set_id` semantics
- dependency shift in `l6.9-budget-runtime` that changes `issue_set_ref` semantics
- supersede, deny, or cancel affecting `validation_run_id`
- budget pressure that invalidates disposable outputs of `validation_runtime` while preserving authoritative rows

## Invalidation law
Invalidation in `validation_runtime` must explicitly name stale records such as `validation_run_id`, `validation_scope`, `rule_set_id`, `issue_set_ref` instead of rebuilding an unnamed mirror.
