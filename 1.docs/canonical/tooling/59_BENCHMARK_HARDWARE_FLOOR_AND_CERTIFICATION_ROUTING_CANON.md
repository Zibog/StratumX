# Benchmark Hardware Floor And Certification Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the singular execution-grade route for hardware-floor certification, baseline/failure/recovery review, and freeze-bound pack routing.
It does not own engine truth and it does not redefine sdk packet families.

## Exact intent schema families
| Intent | Required fields | Output | First denial family |
|---|---|---|---|
| `intent.floor.run_pack` | pack id, hardware profile, baseline id | pack execution verdict | `route.floor.pack_incomplete` |
| `intent.floor.review_baseline` | baseline bundle id, compare horizon, artifact class | baseline review digest | `route.floor.threshold_class_missing` |
| `intent.floor.append_artifacts` | artifact manifest id, failure class, recovery class | append verdict | `route.floor.retained_artifact_gap` |
| `intent.floor.freeze_review` | pack id, blocker family, evidence duty | freeze review verdict | `route.floor.freeze_review_incomplete` |

## Transaction state machine
`selected -> baselined -> executed -> captured -> compared -> recovered -> reexecuted -> freeze_reviewed -> certified / failed`

## Retry and rollback posture
- retry limit: 1 retry for pack launch failure caused by missing non-authoritative mirror only;
- rollback target: restore last-good pack baseline or last legal ladder rung only;
- replay or compare window: `pack horizon`.

## Invalidation triggers
- pack id change;
- threshold class change;
- artifact retention loss;
- blocker family replacement;

## Cache ownership
| Cache | Authoritative content |
|---|---|
| pack cache | active pack definitions and horizons |
| baseline cache | retained baseline/failure/recovery bundles |
| blocker cache | current blocker and recovery mapping |
| artifact ledger | freeze review manifests |

## Artifact ownership
Retained artifacts for this route are:
- retained baseline bundle, failed-run bundle, recovery-run bundle, freeze review manifest.
Failed runs may never overwrite the last-good baseline.

## Diagnostics envelope
Every execution of this route must publish:
- route id, pack id, hardware profile, blocker family, first blocking code, baseline ids, next legal recovery action.

## Recovery action mapping
| Failure code | Next legal recovery action |
|---|---|
| `route.floor.pack_incomplete` | deny execution and route to pack completion review |
| `route.floor.threshold_class_missing` | deny review and route to resource law |
| `route.floor.retained_artifact_gap` | deny freeze until manifest is complete |
| `route.floor.freeze_review_incomplete` | route to release review dashboard with blocker board open |
