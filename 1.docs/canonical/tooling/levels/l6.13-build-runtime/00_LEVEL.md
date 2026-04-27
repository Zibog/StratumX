# Build Runtime

## Role
`build_runtime` owns deterministic build queues, worker claims, and artifact-producing jobs for tooling/editor outputs.

## Owns
- `build_job_id`
- `build_target_set`
- `input_digest`
- `worker_claim_id`
- `build_outcome`

## Consumes
- `l6.6-artifact-plane`
- `l6.7-stream-plane`
- `l6.9-budget-runtime`
- `l6.11-validation-runtime`

## Emits
- built artifacts
- build progress streams
- deterministic manifests for release handoff

## Never owns
- release publication ownership
- editor shell mutation
- hidden build caches outside cache_plane
