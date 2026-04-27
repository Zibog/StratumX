# Causality Capture And Reason Chain Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the singular execution-grade route for reason-chain drilldown, blocker exposure, recovery-step selection, and causal capture routing.
It does not own engine truth and it does not redefine sdk packet families.

## Exact intent schema families
| Intent | Required fields | Output | First denial family |
|---|---|---|---|
| `intent.reason.drilldown` | artifact set, route id, scope | reason-chain bundle | `route.reason.chain_missing` |
| `intent.reason.compare_failed_run` | baseline id, failed-run id, compare class | reason compare digest | `route.reason.compare_gap` |
| `intent.reason.select_recovery_step` | blocking code, ladder state, pack id | recovery-step verdict | `route.reason.recovery_ambiguous` |

## Transaction state machine
`inspected -> chained -> captured -> compared -> routed -> acknowledged`

## Retry and rollback posture
- retry limit: 0 retries for ambiguous recovery selection; 1 retry for missing reason fragment;
- rollback target: no route-level rollback; this route only points to authoritative recovery owners;
- replay or compare window: `bound to upstream pack horizon`.

## Invalidation triggers
- first blocking code change;
- recovery step change;
- compare class change;
- artifact-set loss;

## Cache ownership
| Cache | Authoritative content |
|---|---|
| reason cache | reason-chain fragments and joins |
| recovery cache | selected next-step decisions |
| artifact ledger | retained causal bundles |

## Artifact ownership
Retained artifacts for this route are:
- reason-chain bundle, failed-run compare digest, selected-recovery verdict.
Failed runs may never overwrite the last-good baseline.

## Diagnostics envelope
Every execution of this route must publish:
- route id, source route ids, first blocking code, active ladder step, next legal recovery action.

## Recovery action mapping
| Failure code | Next legal recovery action |
|---|---|
| `route.reason.chain_missing` | request authoritative fragment from owning route |
| `route.reason.compare_gap` | capture failed-run compare digest and rerun review |
| `route.reason.recovery_ambiguous` | deny freeze and route to certification review |
