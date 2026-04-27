# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| proposal_runtime_id | RecordId | stable record identity | unique per record |
| source_session_id | AssistantSessionId | assistant session owning the record | must resolve through assistant_sessions |
| state | LifecycleState | lifecycle state of the record | finite enum only |
| source_digest | SourceDigest | digest of inputs consumed by the record | stable for record lifetime |
| result_ref | ResultRef | ref or pointer to published outcome | bounded and explicit |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `proposal_runtime` without consulting a hidden mirror.
