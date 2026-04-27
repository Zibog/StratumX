# Testing Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

This document defines required test coverage for the canonical `L6 / L6A / L7 / L7A` documentation package.
It closes the documentation-level test contour and names the implementation-facing test classes that any real tooling/editor build must preserve.
It is also the implementation todo-list handoff for executable tooling gold.

## Required test classes

| Test id | Required class | Primary target | Blocking docs | Closure artifact |
|---|---|---|---|---|
| `TEST-TOOLS-001` | authority isolation tests | tiny `L6 authority_core` and no shadow authority owners | `03_ROLE_MAP.md`, `05_DEPENDENCY_MODEL.md`, `14_AUTHORITY_AND_TRANSACTION_MODEL.md`, `21_FORBIDDEN_CONNECTIONS.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-002` | command schema and target-scope tests | command ids, target scope, approval/origin/budget classes | `06_COMMUNICATION_MODEL.md`, `14_AUTHORITY_AND_TRANSACTION_MODEL.md`, `26_SHARED_TYPE_REGISTRY.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-003` | transaction determinism tests | ordered ledger outputs and replay-safe transaction records | `07_THREADING_MODEL.md`, `14_AUTHORITY_AND_TRANSACTION_MODEL.md`, `19_CROSS_LAYER_EXCHANGE_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-004` | apply/revert chain tests | assistant apply/revert and undo bindings through `L6` only | `14_AUTHORITY_AND_TRANSACTION_MODEL.md`, `16_L6A_ASSISTANT_RUNTIME_MODEL.md`, `21_FORBIDDEN_CONNECTIONS.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-005` | snapshot immutability tests | immutable versioned snapshots and swap-safe reads | `13_DATA_PLANE_MODEL.md`, `15_SNAPSHOT_INDEX_DERIVED_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-006` | index rebuild and swap tests | rebuildable indices and scoped replacement | `13_DATA_PLANE_MODEL.md`, `15_SNAPSHOT_INDEX_DERIVED_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-007` | derived non-authority tests | derived projections stay disposable and non-owning | `13_DATA_PLANE_MODEL.md`, `15_SNAPSHOT_INDEX_DERIVED_MODEL.md`, `21_FORBIDDEN_CONNECTIONS.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-008` | artifact manifest and invalidation tests | deterministic artifacts, manifests, and invalidation roots | `14_AUTHORITY_AND_TRANSACTION_MODEL.md`, `15_SNAPSHOT_INDEX_DERIVED_MODEL.md`, `31_ARTIFACT_MANIFEST_LAW.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-009` | stream boundedness tests | bounded stream publication and forward-only behavior | `06_COMMUNICATION_MODEL.md`, `13_DATA_PLANE_MODEL.md`, `19_CROSS_LAYER_EXCHANGE_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-010` | cache eviction and non-authority tests | bounded caches, eviction, and non-truth posture | `13_DATA_PLANE_MODEL.md`, `15_SNAPSHOT_INDEX_DERIVED_MODEL.md`, `20_MEMORY_GPU_DISK_DISCIPLINE.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-011` | workspace non-truth tests | workspace runtime stays coordination-only and non-authoritative | `03_ROLE_MAP.md`, `12_BOUNDARY_PRESERVATION_MATRIX.md`, `21_FORBIDDEN_CONNECTIONS.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-012` | validation legality tests | validation runtime checks legality without owning truth | `03_ROLE_MAP.md`, `12_BOUNDARY_PRESERVATION_MATRIX.md`, `33_BUDGET_RUNTIME_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-013` | preview non-authority tests | preview runtime stays disposable, budgeted, and non-authoritative | `03_ROLE_MAP.md`, `20_MEMORY_GPU_DISK_DISCIPLINE.md`, `21_FORBIDDEN_CONNECTIONS.md`, `35_DEGRADATION_POLICY_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-014` | build reproducibility tests | build runtime emits deterministic build products only | `03_ROLE_MAP.md`, `15_SNAPSHOT_INDEX_DERIVED_MODEL.md`, `31_ARTIFACT_MANIFEST_LAW.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-015` | release packaging legality tests | release runtime stays distinct from build and obeys legality gates | `03_ROLE_MAP.md`, `12_BOUNDARY_PRESERVATION_MATRIX.md`, `31_ARTIFACT_MANIFEST_LAW.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-016` | `L5 -> L6` intake normalization tests | bridge intake types stay typed and non-editor-shaped | `12_BOUNDARY_PRESERVATION_MATRIX.md`, `19_CROSS_LAYER_EXCHANGE_MODEL.md`, `22_L5_SYNCHRONIZATION_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-017` | `L6A` evidence packing tests | bounded evidence packs, redaction, freshness, manifests | `16_L6A_ASSISTANT_RUNTIME_MODEL.md`, `20_MEMORY_GPU_DISK_DISCIPLINE.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-018` | `L6A` proposal staging tests | proposal staging without hidden apply ownership | `16_L6A_ASSISTANT_RUNTIME_MODEL.md`, `21_FORBIDDEN_CONNECTIONS.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-019` | `L6A` lowering tests | lowering from proposals/plans into legal `L6` commands only | `14_AUTHORITY_AND_TRANSACTION_MODEL.md`, `16_L6A_ASSISTANT_RUNTIME_MODEL.md`, `19_CROSS_LAYER_EXCHANGE_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-020` | `L6A` safety gate tests | approval and safety gates before apply | `14_AUTHORITY_AND_TRANSACTION_MODEL.md`, `16_L6A_ASSISTANT_RUNTIME_MODEL.md`, `21_FORBIDDEN_CONNECTIONS.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-021` | `L6A` apply/revert evidence tests | apply/revert emits audit evidence records and rollback bindings | `14_AUTHORITY_AND_TRANSACTION_MODEL.md`, `16_L6A_ASSISTANT_RUNTIME_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-022` | model runtime timeout/cancellation tests | bounded model IO, timeout, cancellation, and request lifecycle | `16_L6A_ASSISTANT_RUNTIME_MODEL.md`, `20_MEMORY_GPU_DISK_DISCIPLINE.md`, `33_BUDGET_RUNTIME_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-023` | `L7` campaign boundary tests | campaign bundles remain cold compiled-control only | `17_L7_STUDIO_ORCHESTRATION_MODEL.md`, `19_CROSS_LAYER_EXCHANGE_MODEL.md`, `21_FORBIDDEN_CONNECTIONS.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-024` | `L7` governance/freeze tests | governance meta and freeze law stay outside frame-level runtime | `17_L7_STUDIO_ORCHESTRATION_MODEL.md`, `23_BUILD_AND_FREEZE_CONDITIONS.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-025` | `L7` reporting traceability tests | reporting outputs remain explicit requests/results and bounded traces | `17_L7_STUDIO_ORCHESTRATION_MODEL.md`, `19_CROSS_LAYER_EXCHANGE_MODEL.md`, `20_MEMORY_GPU_DISK_DISCIPLINE.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-026` | `L7A` goal normalization tests | prompt understanding and bounded goal normalization | `18_L7A_ASSISTANT_BRAIN_MODEL.md`, `19_CROSS_LAYER_EXCHANGE_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-027` | `L7A` planning IR tests | planning IR stays explicit and non-authoritative | `18_L7A_ASSISTANT_BRAIN_MODEL.md`, `21_FORBIDDEN_CONNECTIONS.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-028` | `L7A` canon reasoning tests | canon reasoner emits constrain/deny outputs without direct mutation | `18_L7A_ASSISTANT_BRAIN_MODEL.md`, `21_FORBIDDEN_CONNECTIONS.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-029` | `L7A` optimization and migration tests | optimization and migration stay as alternatives/plans, not direct apply | `18_L7A_ASSISTANT_BRAIN_MODEL.md`, `19_CROSS_LAYER_EXCHANGE_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-030` | `L7A` routing policy tests | model routing stays bounded and policy-driven | `18_L7A_ASSISTANT_BRAIN_MODEL.md`, `33_BUDGET_RUNTIME_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-031` | memory/GPU/disk discipline tests | bounded residency, bounded disk growth, no hidden duplicate stores | `20_MEMORY_GPU_DISK_DISCIPLINE.md`, `33_BUDGET_RUNTIME_MODEL.md`, `35_DEGRADATION_POLICY_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-032` | degradation ladder tests | explicit degrade/defer/deny policy by domain and plane | `32_REPRESENTATION_LADDER_MODEL.md`, `33_BUDGET_RUNTIME_MODEL.md`, `35_DEGRADATION_POLICY_MODEL.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-033` | upper consumer boundary tests | `L8+` editor surfaces consume only public tooling surfaces | `12_BOUNDARY_PRESERVATION_MATRIX.md`, `19_CROSS_LAYER_EXCHANGE_MODEL.md`, `21_FORBIDDEN_CONNECTIONS.md` | `evidence/tests/tooling_test_closure_v4.md` |
| `TEST-TOOLS-034` | family non-ownership tests | families compose planes and sidecars without hidden truth stores | `13_DATA_PLANE_MODEL.md`, `34_DOMAIN_FAMILY_DATA_MODEL.md`, `21_FORBIDDEN_CONNECTIONS.md` | `evidence/tests/tooling_test_closure_v4.md` |

## Closure rule
Testing closure for the documentation package exists only when:
- every required test class above is mapped in the active test-closure artifact
- no required test class is orphaned from its target surfaces
- the active evidence pack resolves `ROOT-022` without referring to superseded package revisions
- every class is specific enough to become CI, bench, integration, or release work without reinterpretation

Any implementation claiming runtime gold must instantiate executable tests that preserve this exact class coverage.

## Canon gold rule
Canon gold for `tooling/` is satisfied when the documentation-package contour is closed, evidence is active, and the runtime-facing implementation worklist is frozen.
Runtime-tooling readiness remains a downstream execution gate for real code, but it does not block canon gold for the documentation package.

## Anti-template audit class
- pairwise diff review across all active levels, sidecars, and families
- owned-record uniqueness review for runtime services and sidecars
- family coordination uniqueness review for every canonical family key
- dependency and invalidation specificity review for adjacent planes and families
