# Release Runtime

## Role
`release_runtime` owns release assembly, packaging manifests, signing/posture records, and publication readiness states.

## Owns
- `release_run_id`
- `release_manifest_id`
- `input_artifact_set`
- `publication_channel`
- `release_outcome`

## Consumes
- `l6.6-artifact-plane`
- `l6.7-stream-plane`
- `l6.9-budget-runtime`
- `l6.13-build-runtime`

## Emits
- release manifests
- publication status streams
- package/signing handoff records

## Never owns
- build queue ownership
- editor widget state
- assistant planning authority
