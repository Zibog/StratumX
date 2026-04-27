# Invalidation

This contract belongs specifically to the release runtime level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `release_runtime`
- dependency shift in `l6.6-artifact-plane` that changes `release_run_id` semantics
- dependency shift in `l6.7-stream-plane` that changes `release_manifest_id` semantics
- dependency shift in `l6.9-budget-runtime` that changes `input_artifact_set` semantics
- dependency shift in `l6.13-build-runtime` that changes `publication_channel` semantics
- supersede, deny, or cancel affecting `release_run_id`
- budget pressure that invalidates disposable outputs of `release_runtime` while preserving authoritative rows

## Invalidation law
Invalidation in `release_runtime` must explicitly name stale records such as `release_run_id`, `release_manifest_id`, `input_artifact_set`, `publication_channel` instead of rebuilding an unnamed mirror.
