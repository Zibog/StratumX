# Heavy Domain Cook Build Canonicalization And Artifact Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define how validated heavy-domain inputs become cooked, packaged, and inspectable artifacts.

## Route stages
| Stage | Required artifact | Failure family | Focus return |
|---|---|---|---|
| cook request | stable cook id and source set | `COOK_REQ_*` | cooker request pane |
| transform | target-specific cooked payload | `COOK_XFORM_*` | transform config row |
| package | runtime artifact bundle | `COOK_PKG_*` | package manifest row |
| publish | artifact manifest and reveal refs | `COOK_PUB_*` | artifact browser |
| invalidate | stale downstream benchmarks and compares | `COOK_INV_*` | affected benchmark or compare pane |

## Artifact law
Every route must publish artifact ids, source lineage, target profile, and first blocker if packaging failed.

## Recovery law
Cook blockers may not be resolved by silently reusing stale artifacts.
