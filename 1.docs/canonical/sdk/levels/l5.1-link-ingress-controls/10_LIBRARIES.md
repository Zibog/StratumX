# Libraries

## Local record classes
- `IngressControlEnvelopeId` for `ingress_control_envelope_id`
- `IngressControlKind` for `control_kind`
- `EngineObjectHandle` for `target_object_handle`
- `EngineRuntimeHandle` for `target_runtime_handle`
- `EngineSessionHandle` for `source_session_handle`
- `SubmissionOrderKey` for `submission_order_key`
- `LegalityGateId` for `legality_gate_id`

## Shared registries consumed
- `compat_versions` registry or lookup surface
- `compat_capabilities` registry or lookup surface
- `transport_policies` registry or lookup surface
- `legality_gates` registry or lookup surface
- `engine_session_handles` registry or lookup surface
- `engine_runtime_handles` registry or lookup surface
- `engine_object_handles` registry or lookup surface

## Library law
`link_ingress_controls` may introduce only record classes that help publish or resolve link ingress controls. It may not invent a parallel semantic vocabulary for adjacent bridge classes.
