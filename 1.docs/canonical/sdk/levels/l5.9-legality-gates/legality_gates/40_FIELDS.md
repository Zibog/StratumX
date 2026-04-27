# Legality Gates Local Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| legality_gate_id | LegalityGateId | required | stable gate identity | unique in registry |
| gate_name | GateName | required | stable machine-readable gate name | must remain API-visible |
| applies_to_control_kinds | IngressControlKindSet | required | control classes checked by this gate | must use declared enums |
| required_capability_set | CompatCapabilitySet | required | capabilities required to pass | must resolve through capability registry |
| deny_reason_code | GateReasonCode | required | machine-readable deny reason emitted on failure | must be finite and enum-based |

## Local invariant rule
Each field above exists because `legality_gates` must publish legality gates without absorbing adjacent semantic truth.
