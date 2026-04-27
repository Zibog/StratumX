# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| validation_run_id | ValidationRunId | stable validation run identity | unique per invocation |
| validation_scope | ValidationScope | what domain or asset was validated | closed enum only |
| rule_set_id | ValidationRuleSetId | rule set applied during the run | must resolve through declared registries |
| issue_set_ref | ValidationIssueSetRef | published issue collection for readers | immutable after run publish |
| outcome_state | ValidationOutcomeState | pass/warn/error/fatal | finite enum only |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `validation_runtime` without consulting a hidden mirror.
