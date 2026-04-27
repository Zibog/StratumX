# Libraries

## Local record classes
- `EngineArtifactRef` for `artifact_ref`
- `ArtifactKind` for `artifact_kind`
- `EngineRuntimeHandle` for `source_runtime_handle`
- `ContentDigest` for `content_digest`
- `ArtifactRetentionPolicy` for `retention_policy`

## Shared registries consumed
- `engine_runtime_handles` registry or lookup surface
- `engine_state_refs` registry or lookup surface

## Library law
`engine_artifact_refs` may introduce only record classes that help publish or resolve engine artifact refs. It may not invent a parallel semantic vocabulary for adjacent bridge classes.
