# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| release_run_id | ReleaseRunId | stable release execution identity | unique per release attempt |
| release_manifest_id | ReleaseManifestId | manifest assembled for publication | must resolve to immutable manifest |
| input_artifact_set | ArtifactSetRef | artifacts admitted into the release | must resolve through artifact_plane |
| publication_channel | PublicationChannel | target publication channel | closed enum only |
| release_outcome | ReleaseOutcome | ready/published/failed/cancelled state | finite enum only |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `release_runtime` without consulting a hidden mirror.
