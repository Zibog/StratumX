# Acceptance Matrix

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines package-level closure conditions for the SDK canonical package root at the active contour `SX-CANON/1.0.24/STACK-v30`.

| Row | Requirement | Evidence ID | Active Artifact | Status |
|-----|-------------|-------------|-----------------|--------|
| ROOT-001 | all critical L5 levels present and indexed | EVID-L5-001 | `evidence/layers/layer_traceability_log_v13.md`, `evidence/layers/layer_completeness_proof_v13.md`, `evidence/layers/layer_local_contract_mesh_v13.md` | pass |
| ROOT-002 | controls split from packets with distinct role boundaries | EVID-L5-002 | `evidence/root/root_traceability_log_v13.md`, `13_TYPOLOGY_SYSTEM.md`, `14_ROLE_CLASS_SEPARATION_MATRIX.md` | pass |
| ROOT-003 | facts split from verdicts with distinct observation/compat boundaries | EVID-L5-003 | `evidence/root/root_traceability_log_v13.md`, `18_RESULT_ARTIFACT_VERDICT_SEPARATION.md` | pass |
| ROOT-004 | artifact refs split from state refs with distinct ref subtypes | EVID-L5-004 | `evidence/root/root_traceability_log_v13.md`, `17_REF_SUBTYPES.md` | pass |
| ROOT-005 | public L4 synchronization is explicit | EVID-L5-005 | `evidence/root/root_traceability_log_v13.md`, `31_ENGINE_L4_BINDING_MAP.md` | pass |
| ROOT-006 | physical data layout model is explicit | EVID-L5-006 | `evidence/root/root_traceability_log_v13.md`, `36_PHYSICAL_DATA_LAYOUT_MODEL.md` | pass |
| ROOT-007 | batch and cursor publication model is explicit | EVID-L5-007 | `evidence/root/root_traceability_log_v13.md`, `37_BATCH_AND_CURSOR_PUBLICATION_MODEL.md` | pass |
| ROOT-008 | compiled gate and compatibility lookup model is explicit | EVID-L5-008 | `evidence/root/root_traceability_log_v13.md`, `38_COMPILED_GATE_PROGRAM_MODEL.md` | pass |
| ROOT-009 | L5 to L6 snapshot and stream alignment is explicit | EVID-L5-009 | `evidence/root/root_traceability_log_v13.md`, `39_SNAPSHOT_ALIGNMENT_WITH_L6.md` | pass |
| ROOT-010 | handle/ref opacity law explicit | EVID-L5-010 | `evidence/root/root_traceability_log_v13.md`, `33_HANDLE_AND_REF_OPACITY_LAW.md` | pass |
| ROOT-011 | no hidden store law explicit | EVID-L5-011 | `evidence/root/root_traceability_log_v13.md`, `evidence/layers/layer_field_invariant_closure_v13.md`, `evidence/layers/layer_local_contract_mesh_v13.md` | pass |
| ROOT-012 | testing model covers required test classes | EVID-L5-012 | `evidence/tests/sdk_test_closure_v13.md`, `24_TESTING_MODEL.md` | pass |
| ROOT-013 | build handoff stays narrow and non-overlapping | EVID-L5-013 | `evidence/root/root_traceability_log_v13.md`, `25_IMPLEMENTATION_HANDOFF.md` | pass |
| ROOT-014 | tools consumption map is explicit | EVID-L5-014 | `evidence/root/root_traceability_log_v13.md`, `34_L5_TO_TOOLS_CONSUMPTION_MAP.md` | pass |
| ROOT-015 | hot-path allocation and locality law is explicit | EVID-L5-015 | `evidence/root/root_traceability_log_v13.md`, `40_L5_HOT_PATH_ALLOCATION_AND_LOCALITY_LAW.md` | pass |
| ROOT-016 | package closure docs exist and are indexed | EVID-L5-016 | `00_INDEX.md`, `27_ACCEPTANCE_MATRIX.md`, `30_EVIDENCE_REGISTRY.md`, `99_AUDIT_READINESS_MATRIX.md` | pass |
| ROOT-017 | authority order is explicit and non-circular | EVID-L5-017 | `evidence/root/root_authority_alignment_v13.md`, `29_DOCUMENT_AUTHORITY_ORDER.md` | pass |
| ROOT-018 | stack version marker is legal and aligned with global canon | EVID-L5-018 | `STACK_VERSION`, `evidence/root/root_traceability_log_v13.md` | pass |
| ROOT-019 | active test execution evidence present | EVID-L5-019 | `evidence/tests/sdk_test_result_posture_v13.md`, `evidence/tests/sdk_test_execution_run_v13.md` | pass |

## Reading
At `SX-CANON/1.0.24/STACK-v30` the SDK package is document-gold clean while remaining implementation-partial relative to runtime breadth.


## Execution-grade extension rows

| Row | Requirement | Evidence ID | Active Artifact | Status |
|-----|-------------|-------------|-----------------|--------|
| ROOT-900 | heavy-domain packet families are field-by-field explicit | EVID-L5-900 | `79_HEAVY_DOMAIN_PACKET_FAMILY_AND_SCHEMA_CONSTITUTION_CANON.md` | pass |
| ROOT-901 | heavy-domain observation classes define rate and delivery law | EVID-L5-901 | `80_HEAVY_DOMAIN_OBSERVATION_RATE_SCOPE_AND_DELIVERY_CONSTITUTION_CANON.md` | pass |
| ROOT-902 | heavy-domain authoring transactions define failures and undo posture | EVID-L5-902 | `81_HEAVY_DOMAIN_AUTHORING_TRANSACTION_AND_FAILURE_CONSTITUTION_CANON.md` | pass |
| ROOT-903 | replay/capture/compare families define retained artifacts and consumers | EVID-L5-903 | `82_HEAVY_DOMAIN_CAPTURE_REPLAY_COMPARE_AND_CONSUMER_CONSTITUTION_CANON.md` | pass |
| ROOT-904 | photoreal proof surfaces publish fallback/certification packets | EVID-L5-904 | `83_PHOTOREAL_DIAGNOSTIC_FALLBACK_AND_CERTIFICATION_PACKET_CATALOG.md` | pass |
| ROOT-905 | first-playable and release-seal lanes have public progress/review packets | EVID-L5-905 | `84_FIRST_PLAYABLE_SLICE_AND_OPERATOR_PROGRESS_SURFACE_CATALOG.md` | pass |
