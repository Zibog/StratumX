# Libraries

## Local record classes
- `LegalityGateId` for `legality_gate_id`
- `GateName` for `gate_name`
- `IngressControlKindSet` for `applies_to_control_kinds`
- `CompatCapabilitySet` for `required_capability_set`
- `GateReasonCode` for `deny_reason_code`

## Shared registries consumed
- `compat_capabilities` registry or lookup surface
- `compat_profiles` registry or lookup surface
- `compat_verdicts` registry or lookup surface

## Library law
`legality_gates` may introduce only record classes that help publish or resolve legality gates. It may not invent a parallel semantic vocabulary for adjacent bridge classes.
