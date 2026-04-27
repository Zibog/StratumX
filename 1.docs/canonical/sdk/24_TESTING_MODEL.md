# Testing Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose

This document defines the StratumX SDK testing model as both:
- the documentation-package closure contour for the L5 bridge canon itself; and
- the exact implementation-facing worklist required before downstream code may claim runtime bridge readiness.

## Required documentation validation classes

| Test ID | Required class | Primary target | Blocking docs | Result artifact |
|---|---|---|---|---|
| `TEST-L5-001` | Lookup correctness | handles, refs, packets, controls, observations, and metrics are mapped through explicit legal lookup surfaces | `31_ENGINE_L4_BINDING_MAP.md`, `38_COMPILED_GATE_PROGRAM_MODEL.md` | `evidence/tests/sdk_test_execution_run_v13.md` |
| `TEST-L5-002` | Pressure testing | hot-path and batch publication laws remain bounded and non-owning | `36_PHYSICAL_DATA_LAYOUT_MODEL.md`, `37_BATCH_AND_CURSOR_PUBLICATION_MODEL.md`, `40_L5_HOT_PATH_ALLOCATION_AND_LOCALITY_LAW.md` | `evidence/tests/sdk_test_execution_run_v13.md` |
| `TEST-L5-003` | Replay testing | observations and metrics remain replay-safe and classification-clean | `18_RESULT_ARTIFACT_VERDICT_SEPARATION.md`, `39_SNAPSHOT_ALIGNMENT_WITH_L6.md` | `evidence/tests/sdk_test_execution_run_v13.md` |
| `TEST-L5-004` | Snapshot swap | public L4 synchronization and snapshot alignment remain explicit | `31_ENGINE_L4_BINDING_MAP.md`, `39_SNAPSHOT_ALIGNMENT_WITH_L6.md` | `evidence/tests/sdk_test_execution_run_v13.md` |
| `TEST-L5-005` | Allocation posture | hot-path publication types remain locality-aware and allocation-bounded | `36_PHYSICAL_DATA_LAYOUT_MODEL.md`, `40_L5_HOT_PATH_ALLOCATION_AND_LOCALITY_LAW.md` | `evidence/tests/sdk_test_execution_run_v13.md` |
| `TEST-L5-006` | Opacity preservation | handles and refs remain opaque and non-leaking | `17_REF_SUBTYPES.md`, `33_HANDLE_AND_REF_OPACITY_LAW.md` | `evidence/tests/sdk_test_execution_run_v13.md` |
| `TEST-L5-007` | Boundary legality | L5 public bridge stays narrow and legal across `L4` and tooling consumers | `12_BOUNDARY_PRESERVATION_MATRIX.md`, `16_BOUNDARY_AUTHORITY.md`, `34_L5_TO_TOOLS_CONSUMPTION_MAP.md` | `evidence/tests/sdk_test_execution_run_v13.md` |
| `TEST-L5-008` | Field invariant | documented field invariants remain explicit and non-contradictory across layers and packet families | `15_FIELD_CONTRACT_RULES.md`, `evidence/layers/layer_field_invariant_closure_v13.md` | `evidence/tests/sdk_test_execution_run_v13.md` |

## Implementation-facing extension classes

The following extension classes are part of the runtime-bridge implementation todo list and must be executable in downstream code/test pipelines:
- packet publication legality tests
- control publication legality tests
- legality lookup correctness tests
- ingress ordering tests
- observation batch immutability tests
- metric batch immutability tests
- cursor progression and lag-bound tests
- handle/ref opacity stress tests
- artifact ref opacity tests
- no hidden store tests
- invalidation and epoch rollover tests
- L5 to tooling compatibility soak tests

## Closure rule

Canon testing closure exists only when:
- all eight required validation classes above have executed verdicts;
- the executed verdicts are recorded in the active execution posture artifact;
- the execution posture artifact is active in the evidence registry;
- acceptance and readiness rows for active execution evidence are `pass`;
- the implementation-facing extension classes are frozen as an executable worklist.

Downstream runtime-bridge readiness additionally requires execution of the implementation-facing extension classes, but that downstream execution is not a blocker for canon gold.

## Anti-template audit class
- pairwise diff review across all L5 local contract packs
- field-class uniqueness review for each semantic class
- dependency and sync-surface uniqueness review for adjacent levels
