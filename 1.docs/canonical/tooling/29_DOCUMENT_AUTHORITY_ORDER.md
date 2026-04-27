# Document Authority Order

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Authority order
1. constitutions
2. root documents
3. level contracts
4. family contracts and `36_CANONICAL_FAMILY_REGISTRY.md`
5. active evidence pack
6. archival notes and superseded evidence records

## Active evidence pack for this package version
The active evidence pack is exactly the active contour declared by `30_EVIDENCE_REGISTRY.md`:
- `evidence/root/root_traceability_log_v4.md`
- `evidence/layers/layer_traceability_log_v4.md`
- `evidence/root/l6_plane_separation_proof_v4.md`
- `evidence/root/tooling_evidence_authority_transaction_v4.md`
- `evidence/root/tooling_evidence_runtime_services_v4.md`
- `evidence/root/tooling_evidence_stack_compatibility_v4.md`
- `evidence/root/tooling_evidence_budget_degradation_v4.md`
- `evidence/root/tooling_evidence_family_mesh_v4.md`
- `evidence/root/tooling_evidence_sidecar_mesh_v4.md`
- `evidence/root/tooling_evidence_build_release_mesh_v4.md`
- `evidence/tests/tooling_test_closure_v4.md`
- `evidence/tests/tooling_test_result_posture_v4.md`
- `evidence/tests/tooling_test_execution_run_v4.md`

## Family identity rule
Canonical family identity is defined by `36_CANONICAL_FAMILY_REGISTRY.md` and the `Canonical family:` line inside each family-declared `00_LEVEL.md` contract in the active package.
Physical folder ordinals under `families/` are locality labels only and may not be used as authoritative family identifiers.

## Pre-v3 rule
Package revisions before `v3` did not contain an authoritative evidence pack for current closure.
They are superseded by the active pack above and may not be used to claim current acceptance or readiness.

## Derivative rule
`27_ACCEPTANCE_MATRIX.md` and `99_AUDIT_READINESS_MATRIX.md` may claim `pass` only through the active evidence pack above and the canonical family registry above.
They must not backfill missing proof from superseded package revisions.

## Conflict rule
If an archival note disagrees with the active evidence pack, the archival note loses authority immediately.
If the active evidence pack disagrees with constitutions, root docs, local contracts, or the canonical family registry, the evidence pack is wrong and must be regenerated.


## Local contract mesh authority
- `evidence/layers/tooling_level_local_contract_mesh_v4.md` is the active file-by-file completeness artifact for levels and sidecars.
- `evidence/layers/tooling_family_local_contract_mesh_v4.md` is the active file-by-file completeness artifact for families.
- these artifacts support, but do not replace, the root evidence pack listed above.
