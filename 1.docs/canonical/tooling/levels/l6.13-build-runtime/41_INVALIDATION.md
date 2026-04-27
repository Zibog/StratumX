# Invalidation

This contract belongs specifically to the build runtime level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `build_runtime`
- dependency shift in `l6.6-artifact-plane` that changes `build_job_id` semantics
- dependency shift in `l6.7-stream-plane` that changes `build_target_set` semantics
- dependency shift in `l6.9-budget-runtime` that changes `input_digest` semantics
- dependency shift in `l6.11-validation-runtime` that changes `worker_claim_id` semantics
- supersede, deny, or cancel affecting `build_job_id`
- budget pressure that invalidates disposable outputs of `build_runtime` while preserving authoritative rows

## Invalidation law
Invalidation in `build_runtime` must explicitly name stale records such as `build_job_id`, `build_target_set`, `input_digest`, `worker_claim_id` instead of rebuilding an unnamed mirror.
