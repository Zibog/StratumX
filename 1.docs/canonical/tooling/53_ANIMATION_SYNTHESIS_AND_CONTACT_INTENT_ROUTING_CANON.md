# Animation Synthesis And Contact Intent Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the singular execution-grade route for animation contact synthesis, micro-motion, and soft-surface intent routing.
It does not own engine truth and it does not redefine sdk packet families.

## Exact intent schema families
| Intent | Required fields | Output | First denial family |
|---|---|---|---|
| `intent.animation.contact_compare` | contact set id, baseline id, compare class | contact compare digest | `route.anim.contact_compare_missing` |
| `intent.animation.micro_motion_review` | synthesis profile id, rung, scope | micro-motion legality digest | `route.anim.micro_motion_scope_invalid` |
| `intent.animation.restore_contact` | baseline bundle id, restore class, rung | recovery plan | `route.anim.restore_required` |

## Transaction state machine
`requested -> validated -> resolved_scope -> applied -> captured -> compared -> reviewed -> certified / rolled_back`

## Retry and rollback posture
- retry limit: 1 retry for missing non-authoritative mirror only;
- rollback target: restore declared contact baseline or synthesis rung only;
- replay or compare window: `900 frames`.

## Invalidation triggers
- contact set replacement;
- synthesis profile change;
- solver rung change;
- wetness-response class change;

## Cache ownership
| Cache | Authoritative content |
|---|---|
| contact cache | contact sets and compare digests |
| synthesis cache | micro-motion profile state |
| artifact ledger | baseline/failure/recovery bundles |

## Artifact ownership
Retained artifacts for this route are:
- contact compare digest, synthesis rung trace, recovery bundle.
Failed runs may never overwrite the last-good baseline.

## Diagnostics envelope
Every execution of this route must publish:
- route id, contact scope, active rung, first blocking code, baseline ids, next legal recovery action.

## Recovery action mapping
| Failure code | Next legal recovery action |
|---|---|
| `route.anim.contact_compare_missing` | capture contact baseline and rerun compare |
| `route.anim.micro_motion_scope_invalid` | normalize scope and rerun micro-motion review |
| `route.anim.restore_required` | restore last-good contact baseline and rerun compare |
