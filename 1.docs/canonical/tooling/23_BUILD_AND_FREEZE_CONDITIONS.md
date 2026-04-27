# Build And Freeze Conditions

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

The tooling canon stack is locally document-gold only when:
- `L6 authority_core` remains tiny and single-writer
- all editor mutation paths go through `L6` commands and transactions
- command classes, invalidation classes, and transaction outputs are explicit
- every heavy domain answers authority/snapshot/index/derived/artifact/cache separation
- artifact plane is explicit and distinct from cache and derived
- build runtime is explicit and distinct from release runtime
- budget runtime is explicit and enforces pressure policy
- `L6A` has no direct mutation authority and no shadow world
- `L7A` has no direct apply authority
- `L7` remains cold compiled-control only
- all shortcut paths are denied
- memory/GPU/disk discipline is explicit
- the active tooling-local evidence pack is present and authoritative for local closure
- `27_ACCEPTANCE_MATRIX.md` has no `fail` row and any historical-local freshness is declared honestly
- `99_AUDIT_READINESS_MATRIX.md` contains no unresolved blocker that contradicts local document gold
- `24_TESTING_MODEL.md` is closed by the active test-closure artifact
- the implementation-facing tooling target matrix is explicit enough to become CI and task-graph work without reinterpretation
- every declared level, sidecar, and family has non-stub local contracts, verified by active local contract mesh artifacts
- `36_CANONICAL_FAMILY_REGISTRY.md` is complete and authoritative for family identity and completeness
- active test-result artifact exists with executed documentation-package evidence

Umbrella production-gold requires more: rebasing the tooling-local v3 evidence contour and keeping the honesty notes in `27`, `30`, and `99` intact.

Implementation runtime-tooling readiness is tracked separately and is not a blocker for local document gold.
Any package revision missing the active evidence pack or the active test-closure artifact is not even locally gold, even if the structural docs look complete.

Registry-driven family resolution is mandatory for automation; physical folder ordinals are non-authoritative and not a blocker when canonical family keys are unique and active.

## Anti-template rule
A tooling local pack is not complete if another level or family can swap in the same local `20_DEPENDENCIES.md` files across levels and families, local `30_COMMUNICATION.md` files across levels and families, local `40_FIELDS.md` files across levels and families, or local invalidation docs such as `41_INVALIDATION.md` and `41_ACTIVATION_AND_INVALIDATION.md` without changing legal edges, owned records, or degradation behavior. Every active level and family must remain semantically distinguishable under pairwise diff.
