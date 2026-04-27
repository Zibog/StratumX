# Testing Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose

This document defines the **documentation-package** testing model for the StratumX engine canon.
It governs validation of the canonical engine document set itself.
It does **not** claim compiled runtime, benchmark, or CI execution for workspace crates.

## Scope

This testing model covers:
- crate-test obligation registration against `constitutions/STRATUMX_CRATE_TEST_MATRIX.md`;
- layer-test obligation registration against `constitutions/STRATUMX_LAYER_TEST_MATRIX.md`;
- package-level legality of root testing, evidence, and freeze bindings;
- execution-posture registration for document-package gold closure.

It does not replace implementation-facing crate test suites.
Those remain downstream of the canonical package and are governed by the constitutional test matrices.

## Required documentation validation classes

| Test ID | Required validation class | Primary target | Blocking docs | Result artifact |
|---|---|---|---|---|
| `TEST-ENG-001` | crate-matrix registration validation | crate-level mandatory test obligations are explicitly registered | `constitutions/STRATUMX_CRATE_TEST_MATRIX.md`, `15_EVIDENCE_REGISTRY.md` | `evidence/tests/engine_test_execution_run_v1.md` |
| `TEST-ENG-002` | layer-matrix registration validation | layer-level mandatory test obligations are explicitly registered | `constitutions/STRATUMX_LAYER_TEST_MATRIX.md`, `15_EVIDENCE_REGISTRY.md` | `evidence/tests/engine_test_execution_run_v1.md` |
| `TEST-ENG-003` | root testing-model legality validation | engine testing model stays package-scoped and cites the governing constitutional matrices | `17_TESTING_MODEL.md`, `constitutions/STRATUMX_TESTING_CONSTITUTION.md` | `evidence/tests/engine_test_execution_run_v1.md` |
| `TEST-ENG-004` | freeze-gate legality validation | gold freeze remains blocked when execution evidence is absent or failed | `18_BUILD_AND_FREEZE_CONDITIONS.md`, `16_AUDIT_READINESS_MATRIX.md` | `evidence/tests/engine_test_execution_run_v1.md` |
| `TEST-ENG-005` | execution-evidence artifact validation | test execution posture artifact exists and records an executed result set | `evidence/tests/engine_test_result_posture_v1.md`, `15_EVIDENCE_REGISTRY.md` | `evidence/tests/engine_test_execution_run_v1.md` |
| `TEST-ENG-006` | acceptance/evidence/readiness linkage validation | `ROOT-021`, `EVID-ROOT-021`, and `ENG-R-021` remain mechanically aligned | `14_ACCEPTANCE_MATRIX.md`, `15_EVIDENCE_REGISTRY.md`, `16_AUDIT_READINESS_MATRIX.md` | `evidence/tests/engine_test_execution_run_v1.md` |

## Closure rule

Documentation-package testing closure exists only when:
- every required validation class above has an executed verdict;
- every executed verdict is recorded in the active execution posture artifact;
- the execution posture artifact is referenced by acceptance, evidence, and readiness without contradiction;
- engine gold freeze remains blocked whenever an executed validation fails.

## Notes

The engine canonical package may be gold as a **document package** while downstream code/runtime execution remains an implementation concern.
Implementation-facing crate tests must still preserve the constitutional obligations registered in the crate and layer test matrices.
