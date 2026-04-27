# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| artifact_id | ArtifactId | stable artifact identity | unique and immutable |
| artifact_kind | ArtifactKind | closed artifact kind enum | must use declared enum |
| source_digest | SourceDigest | digest of source inputs used to create artifact | stable for artifact lifetime |
| content_digest | ContentDigest | digest of artifact payload | stable after publish |
| retention_class | ArtifactRetentionClass | retention/disposal posture | finite enum only |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `artifact_plane` without consulting a hidden mirror.
