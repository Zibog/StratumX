# Artifact Plane

## Role
`artifact_plane` tracks immutable tooling artifacts such as baked manifests, generated files, and deterministic outputs.

## Owns
- `artifact_id`
- `artifact_kind`
- `source_digest`
- `content_digest`
- `retention_class`

## Consumes
- `l6.3-snapshot-plane`
- `l6.13-build-runtime`
- `l6.14-release-runtime`

## Emits
- artifact manifests
- reverse references to generated outputs
- deterministic output lookup surfaces

## Never owns
- cache rows
- live preview buffers
- editor-local temporary widgets
