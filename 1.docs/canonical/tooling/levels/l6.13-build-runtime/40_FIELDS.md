# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| build_job_id | BuildJobId | stable build job identity | unique per queued build |
| build_target_set | BuildTargetSet | declared targets for the job | must be explicit and finite |
| input_digest | SourceDigest | digest of build inputs | must be stable for the job run |
| worker_claim_id | WorkerClaimId | worker claim on the job | single active claim at a time |
| build_outcome | BuildOutcome | succeeded/failed/cancelled state | finite enum only |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `build_runtime` without consulting a hidden mirror.
