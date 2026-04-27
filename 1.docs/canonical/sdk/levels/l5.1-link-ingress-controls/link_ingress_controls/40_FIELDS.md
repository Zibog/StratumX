# Link Ingress Controls Local Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| ingress_control_envelope_id | IngressControlEnvelopeId | required | stable identity of one submitted control envelope | unique within one engine session |
| control_kind | IngressControlKind | required | closed control kind enum | must use the shared registry control kind enum |
| target_object_handle | EngineObjectHandle | optional | addressed engine object when the control is object-scoped | must resolve through `engine_object_handles` when present |
| target_runtime_handle | EngineRuntimeHandle | required | addressed public runtime surface | must resolve through `engine_runtime_handles` |
| source_session_handle | EngineSessionHandle | required | source bridge session | must resolve through `engine_session_handles` |
| submission_order_key | SubmissionOrderKey | required | ordered write-side publication key | monotonic per session and control domain |
| legality_gate_id | LegalityGateId | required | gate profile used before control acceptance | must resolve through `legality_gates` |

## Local invariant rule
Each field above exists because `link_ingress_controls` must publish link ingress controls without absorbing adjacent semantic truth.
