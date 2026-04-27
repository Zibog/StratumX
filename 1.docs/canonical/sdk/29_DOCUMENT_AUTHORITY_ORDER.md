# Document Authority Order

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose

This document establishes the formal authority hierarchy for resolving conflicts within the SDK canonical package.

## Authority Hierarchy

The following is the strict authority order for SDK package documents. Higher items override lower items.

| Authority Level | Source | Scope | Overrides |
|-----------------|--------|-------|-----------|
| 1 | Global Constitutional Law | Stack-wide (from global umbrella authority order (program-context authority)) | All SDK package documents when stack-wide rules apply |
| 2 | SDK Constitutions | SDK-specific constitutional laws (if any) | All SDK package documents except global constitutional law |
| 3 | SDK Root Documents | Package root docs (00-40 series) | SDK acceptance matrices and local documents |
| 4 | `27_ACCEPTANCE_MATRIX.md` | SDK acceptance criteria | SDK evidence registry and readiness matrix |
| 5 | `30_EVIDENCE_REGISTRY.md` | SDK active evidence artifacts | SDK readiness matrix when defining active artifacts |
| 6 | Active Evidence Artifacts (v13) | Package-contained proof artifacts | Superseded evidence versions and local notes |
| 7 | `99_AUDIT_READINESS_MATRIX.md` | SDK readiness verification | Local notes and observations |
| 8 | Level Contracts | L5 level-specific documents | Local derived notes within that level |
| 9 | Local Derived Notes | Level-specific annotations | None (lowest authority) |

## Authority Resolution Principles

1. **Global Supremacy**: Global canonical umbrella documents in the parent canonical root (`../`) override SDK package documents when defining stack-wide relationships, dependencies, and boundaries.

2. **Root Document Supremacy**: SDK root documents (00-40 series) override acceptance and evidence documents for package-specific matters.

3. **Acceptance Matrix Supremacy**: `27_ACCEPTANCE_MATRIX.md` defines what constitutes gold readiness for SDK, overriding evidence registry when they conflict on acceptance criteria.

4. **Evidence Registry Supremacy**: `30_EVIDENCE_REGISTRY.md` defines what constitutes an active artifact, overriding readiness matrix when they conflict on artifact status.

5. **Active Evidence Contour Supremacy**: Only v13 evidence artifacts are authoritative. Superseded versions (v9, v10, v11) are non-authoritative and may not be used to satisfy acceptance criteria.

6. **Readiness Matrix Subordination**: `99_AUDIT_READINESS_MATRIX.md` derives its authority from acceptance matrix and evidence registry; it may not self-certify.

## Prohibited Authority Patterns

The following authority patterns are explicitly prohibited:

1. **Circular Authority**: No document may derive its authority from a document it itself authorizes
2. **Evidence Version Shopping**: Selecting older evidence versions to satisfy acceptance criteria when newer versions exist
3. **Self-Certification**: Readiness matrix may not certify itself without referencing acceptance rows and evidence IDs
4. **Hidden Truth**: Level contracts may not claim authority over truth owned by engine package
5. **Bypass Global Canon**: SDK documents may not contradict global dependency model or boundary preservation rules

## Conflict Resolution Process

When a conflict arises between SDK documents:
1. Identify the authority level of each conflicting document using the table above
2. The document with the higher authority level prevails
3. If both documents are at the same authority level, escalate to global canonical umbrella
4. If still unresolved, flag for constitutional review

## Version Discipline

Only evidence artifacts from the active contour (v13) are authoritative.
Previous versions (v9, v10, v11) are superseded and may not be used to satisfy current gold closure requirements.

The active evidence contour is declared in `30_EVIDENCE_REGISTRY.md` and may only be changed through formal evidence contour migration.
