# Engine Artifact Refs Local Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| artifact_ref | EngineArtifactRef | required | stable ref to one immutable engine artifact | must remain opaque and immutable |
| artifact_kind | ArtifactKind | required | closed artifact class enum | must use declared enum |
| source_runtime_handle | EngineRuntimeHandle | required | runtime surface that produced the artifact | must resolve through `engine_runtime_handles` when runtime-produced |
| content_digest | ContentDigest | required | digest of the immutable artifact payload | must be stable for the artifact lifetime |
| retention_policy | ArtifactRetentionPolicy | required | declared retention/disposal class | must be explicit and finite |

## Local invariant rule
Each field above exists because `engine_artifact_refs` must publish engine artifact refs without absorbing adjacent semantic truth.
