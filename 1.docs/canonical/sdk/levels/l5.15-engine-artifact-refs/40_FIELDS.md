# Fields

## Canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| artifact_ref | EngineArtifactRef | required | stable ref to one immutable engine artifact | must remain opaque and immutable |
| artifact_kind | ArtifactKind | required | closed artifact class enum | must use declared enum |
| source_runtime_handle | EngineRuntimeHandle | required | runtime surface that produced the artifact | must resolve through `engine_runtime_handles` when runtime-produced |
| content_digest | ContentDigest | required | digest of the immutable artifact payload | must be stable for the artifact lifetime |
| retention_policy | ArtifactRetentionPolicy | required | declared retention/disposal class | must be explicit and finite |

## No hidden store law
All semantic truth in `engine_artifact_refs` must be visible as one of the declared fields above or in a declared lower-layer dependency. Hidden caches, shadow graphs, or side mirrors are illegal.
