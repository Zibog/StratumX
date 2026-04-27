# Global Build And Freeze Conditions

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines when the entire canonical archive may be treated as globally frozen.
It separates local document cleanliness from umbrella production-gold freeze.

## Legal freeze classes
### Local document freeze
A package may claim a local document-freeze when its active tree is package-pure, ordinal-clean, and internally non-contradictory.
That claim does not automatically imply current implementation proof or umbrella production-gold.

### Global production-gold freeze
Global production-gold freeze is legal only when all of the following are true:
- `01_SCOPE.md` is satisfied without exception;
- every row in `06_GLOBAL_ACCEPTANCE_MATRIX.md` is `pass`;
- every evidence ID in `07_GLOBAL_EVIDENCE_REGISTRY.md` resolves to a `current` physical artifact, or a mixed artifact explicitly allowed by the corresponding acceptance row;
- every row in `99_GLOBAL_AUDIT_READINESS_MATRIX.md` is `pass`;
- the package authority-root and constitutional sets are complete across `engine`, `sdk`, `tooling`, `world`, and `editor`;
- the continuous legal relay path `engine -> sdk -> tooling -> editor` is proven by the active closed relay manifest;
- the active contradiction matrix shows no contradiction between umbrella law and package root law;
- the package pass-chain is current enough for umbrella use;
- the global stack version marker is present, singular, and valid across all active documents;
- the editor base shell is clean, non-demo, and sufficient for day-zero terrain/material/sky authoring without lab-only detours; and
- `31_GOLD_STATUS_STRATIFICATION_CANON.md` allows the stronger production claim.

## Prohibited freeze conditions
Global production-gold freeze is prohibited if:
- any global acceptance row is not `pass`;
- any global evidence artifact is missing, contradictory, or unresolved;
- any global readiness row is not `pass`;
- any package root-set or constitutional-set completeness proof is incomplete;
- any hot/core level manifest is incomplete, contradictory, or unresolved;
- the continuous legal relay path is broken or not provable;
- contradiction exists between umbrella dependency/boundary law and package root law;
- the package pass-chain is only historical-local or otherwise non-current for umbrella use;
- any active document remains on a superseded stack version marker;
- the base authoring shell still promotes demo/debug-only controls ahead of day-zero world creation; or
- the reality/workflow ledgers still contradict a production-gold boast.

## Verification sequence
1. verify package root-set and constitutional-set completeness;
2. verify package purity and active ordinal uniqueness;
3. verify hot/core density closure across all packages through the explicit level manifest;
4. verify global evidence resolution;
5. verify global readiness alignment row by row;
6. verify continuous stack relay closure through the artifact-by-artifact relay manifest;
7. verify contradiction-free dependency and boundary law through the explicit contradiction matrix;
8. verify current stack version alignment across the full active contour;
9. verify the material-first day-zero editor conveyor from project to validated world;
10. verify that reality and workflow ledgers no longer expose open production blockers.

## Emergency unfreeze conditions
If after freeze any active evidence artifact becomes missing, inactive for umbrella use, contradictory, constitutionally illegal, or the base shell regresses into prototype/debug clutter, the stack immediately exits production-gold freeze and re-enters remediation.
