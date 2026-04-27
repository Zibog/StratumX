# Fields

## Canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| legality_gate_id | LegalityGateId | required | stable gate identity | unique in registry |
| gate_name | GateName | required | stable machine-readable gate name | must remain API-visible |
| applies_to_control_kinds | IngressControlKindSet | required | control classes checked by this gate | must use declared enums |
| required_capability_set | CompatCapabilitySet | required | capabilities required to pass | must resolve through capability registry |
| deny_reason_code | GateReasonCode | required | machine-readable deny reason emitted on failure | must be finite and enum-based |

## No hidden store law
All semantic truth in `legality_gates` must be visible as one of the declared fields above or in a declared lower-layer dependency. Hidden caches, shadow graphs, or side mirrors are illegal.
