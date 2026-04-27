# Invalidation

This contract belongs specifically to the artifact plane level and may not be reused verbatim by another tooling level.


## Invalidation triggers for `artifact_plane`
- dependency shift in `l6.3-snapshot-plane` that changes `artifact_id` semantics
- dependency shift in `l6.13-build-runtime` that changes `artifact_kind` semantics
- dependency shift in `l6.14-release-runtime` that changes `source_digest` semantics
- supersede, deny, or cancel affecting `artifact_id`
- budget pressure that invalidates disposable outputs of `artifact_plane` while preserving authoritative rows

## Invalidation law
Invalidation in `artifact_plane` must explicitly name stale records such as `artifact_id`, `artifact_kind`, `source_digest`, `content_digest` instead of rebuilding an unnamed mirror.
