# Global Authority Order

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Authority Hierarchy

The following is the strict, unambiguous authority order for resolving conflicts within the StratumX canonical stack. Higher items override lower items.

| Authority level | Source | Scope | Overrides |
|---|---|---|---|
| 1 | Engine constitutional law | Engine-specific domains such as performance, memory, benchmark, determinism, or other constitution-governed runtime domains | All global and package documents when the constitution directly governs that domain |
| 2 | `05_GLOBAL_AUTHORITY_ORDER.md` | Stack-wide authority ordering and conflict resolution | All package root documents and package-local laws when stack-wide ordering is at issue |
| 3 | Package constitutional sets and package root laws | Package-specific law inside `engine`, `sdk`, `tooling`, or `editor` | Package acceptance/evidence/readiness and local contracts inside that package |
| 4 | `06_GLOBAL_ACCEPTANCE_MATRIX.md` | Stack-wide acceptance requirements | Package acceptance matrices when stack-wide gold is being judged |
| 5 | Package acceptance matrix | Package-specific acceptance requirements | Package evidence and readiness rows inside that package |
| 6 | `07_GLOBAL_EVIDENCE_REGISTRY.md` | Stack-wide active evidence routing | Package-level evidence routing only when stack-wide evidence status is at issue |
| 7 | Package evidence registry | Package-specific active evidence routing | Package readiness rows inside that package |
| 8 | `99_GLOBAL_AUDIT_READINESS_MATRIX.md` | Stack-wide readiness verification | Package readiness assessments when stack-wide gold is being judged |
| 9 | Package readiness matrix | Package-specific readiness verification | Package-local notes or observations |
| 10 | Local notes or observations | Lowest authority commentary | Nothing |

## Authority Resolution Principles

1. **Engine constitutional supremacy** applies only when a specific engine constitution explicitly governs the disputed domain.
2. **Global umbrella supremacy** applies when the dispute is about stack-wide dependency legality, authority ordering, or inter-package boundary law.
3. **Package constitutional authority** means each package may carry constitutional authority as a **set** of active constitutional/root-law documents; `sdk`, `tooling`, and `editor` do not need one monolithic constitution file to have constitutional authority.
4. **Acceptance supremacy** means global acceptance defines stack-wide gold, while package acceptance defines package-local gold.
5. **Evidence supremacy** means active evidence routing is controlled by the corresponding evidence registry at the scope being judged.
6. **Readiness supremacy** means readiness may only validate rows that already exist in the corresponding acceptance/evidence contour.

## Conflict Resolution Process

1. Identify the authority level of each conflicting document using the table above.
2. The document with the higher authority level prevails.
3. If both documents are at the same authority level, the more specific document prevails for package-internal matters, while the umbrella document prevails for stack-wide matters.
4. If the conflict still cannot be resolved, escalate to engine constitutional law if the domain is constitution-governed; otherwise mark the stack non-gold until the contradiction is repaired.

## Required Package Authority-Root Set

Each package must expose an authority-root set composed of active root documents that cover at minimum:
- package index
- scope
- stack or layer map
- role map
- dependency model
- boundary preservation matrix
- acceptance matrix
- evidence registry
- build and freeze conditions
- document authority order
- audit readiness matrix

Concrete ordinals differ by package and must be taken from the package root itself, not from a fake shared numbering scheme.
