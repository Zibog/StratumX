# Large World And Streaming Authoring Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the singular execution-grade route for large-world streaming, residency, transition, and long-range visibility authoring.
It does not own engine truth and it does not redefine sdk packet families.

## Exact intent schema families
| Intent | Required fields | Output | First denial family |
|---|---|---|---|
| `intent.world.inspect_residency` | cell scope, stream tier, compare horizon | residency slice + diagnostics | `route.world.residency_scope_invalid` |
| `intent.world.compare_transition` | transition id, baseline id, artifact class | transition compare digest | `route.world.transition_compare_missing` |
| `intent.world.certify_floor` | pack id, threshold class, baseline bundle | floor-ready certification verdict | `route.world.floor_pack_incomplete` |

## Transaction state machine
`requested -> validated -> resolved_scope -> applied -> captured -> compared -> reviewed -> certified / rolled_back`

## Retry and rollback posture
- retry limit: 2 retries for transient artifact read or compare digest assembly failure only;
- rollback target: restore to declared residency baseline or transition anchor only;
- replay or compare window: `1800 frames`.

## Invalidation triggers
- residency radius change;
- cell assignment change;
- stream frontier rebuild;
- baseline pointer replacement;

## Cache ownership
| Cache | Authoritative content |
|---|---|
| residency cache | current residency buckets |
| transition cache | declared transition compare digests |
| artifact ledger | retained baseline/failure/recovery bundles |

## Artifact ownership
Retained artifacts for this route are:
- residency slice, transition compare digest, pressure ladder trace.
Failed runs may never overwrite the last-good baseline.

## Diagnostics envelope
Every execution of this route must publish:
- route id, scope, current ladder step, first blocking code, baseline ids, next legal recovery action.

## Recovery action mapping
| Failure code | Next legal recovery action |
|---|---|
| `route.world.residency_scope_invalid` | normalize to declared cell scope and rerun route |
| `route.world.transition_compare_missing` | capture transition baseline and rerun compare |
| `route.world.floor_pack_incomplete` | deny freeze and route to certification review |
