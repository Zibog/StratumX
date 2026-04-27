# Compatibility Verdicts Local Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| compat_verdict_id | CompatVerdictId | required | stable identity of one compatibility decision | unique per evaluation request |
| evaluated_profile_id | CompatProfileId | required | profile that was evaluated | must resolve through `compat_profiles` |
| evaluated_version_id | CompatVersionId | required | version presented for evaluation | must resolve through `compat_versions` |
| verdict_state | CompatVerdictState | required | allow/warn/deny result | must use declared enum only |
| reason_code_set | CompatReasonCodeSet | required | bounded machine-readable explanation set | must be enum-based and finite |

## Local invariant rule
Each field above exists because `compat_verdicts` must publish compatibility verdicts without absorbing adjacent semantic truth.
