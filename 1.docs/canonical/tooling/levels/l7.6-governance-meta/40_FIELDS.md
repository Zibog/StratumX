# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| governance_meta_record_id | RecordId | stable record identity | unique per published record |
| scope_id | ScopeId | scope of the campaign/reasoning record | explicit and bounded |
| state | LifecycleState | cold lifecycle state | finite enum only |
| input_ref_set | InputRefSet | refs consumed by the record | must stay explicit |
| output_ref_set | OutputRefSet | refs emitted by the record | must stay explicit |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `governance_meta` without consulting a hidden mirror.
