# Dream Stack Asset Canonicalization And Import Validation Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact routing stages for heavy-domain asset intake before any cook or certification work begins.

## Route stages
| Stage | Required output | Blocker family | Focus return |
|---|---|---|---|
| source intake | source ref, lineage digest, claimed family | `IMP_*` | source picker or path field |
| family normalization | canonical family id, variant posture | `AST_FAM_*`, `NRM_*` | family selector |
| schema validation | declared metadata and required fields | `AST_SCHEMA_*` | exact schema row |
| canonical id issue | stable asset id and source linkage | `AST_ID_*` | identity row |
| downstream invalidation | affected cooks, compares, and labs marked dirty | `AST_INV_*` | affected lab or cooker |

## Artifact law
Import routing must publish:
- source lineage ref;
- canonical family id;
- normalization verdict;
- first blocker code if blocked.

## Focus return law
On failure, focus returns to the exact source field, family selector, or metadata row that caused the blocker.
